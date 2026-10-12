--- mobile/link/src/bin/relay/privileges.rs.orig	2026-10-11 11:41:41 UTC
+++ mobile/link/src/bin/relay/privileges.rs
@@ -3,7 +3,7 @@ pub(super) fn drop_root() -> io::Result<()> {
 use std::io;
 
 pub(super) fn drop_root() -> io::Result<()> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         // This function is called only in the single-threaded startup path.
         // getpwnam's borrowed record is consumed before any other libc lookup.
@@ -20,6 +20,7 @@ pub(super) fn drop_root() -> io::Result<()> {
             if uid == 0 || gid == 0 {
                 return Err(io::Error::other("unprivileged_account_required"));
             }
+            #[cfg(target_os = "linux")]
             if libc::setgroups(0, std::ptr::null()) != 0
                 || libc::setgid(gid) != 0
                 || libc::setuid(uid) != 0
@@ -30,10 +31,26 @@ pub(super) fn drop_root() -> io::Result<()> {
             {
                 return Err(io::Error::other("privilege_drop_failed"));
             }
+            #[cfg(target_os = "freebsd")]
+            {
+                let mut trace: libc::c_int = libc::PROC_TRACE_CTL_DISABLE;
+                let mut nnp: libc::c_int = libc::PROC_NO_NEW_PRIVS_ENABLE;
+
+                if libc::setgroups(0, std::ptr::null()) != 0
+                    || libc::setgid(gid) != 0
+                    || libc::setuid(uid) != 0
+                    || libc::procctl(libc::P_PID, 0, libc::PROC_NO_NEW_PRIVS_CTL, &mut nnp as *mut _ as *mut libc::c_void) != 0
+                    || libc::procctl(libc::P_PID, 0, libc::PROC_TRACE_CTL, &mut trace as *mut _ as *mut libc::c_void) != 0
+                    || libc::geteuid() != uid
+                    || libc::getegid() != gid
+                {
+                    return Err(io::Error::other("privilege_drop_failed"));
+                }
+            }
         }
         Ok(())
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         Err(io::Error::other("linux_required"))
     }
