--- nebula_app/src/ai_hook/unix.rs.orig	2026-10-07 15:23:24 UTC
+++ nebula_app/src/ai_hook/unix.rs
@@ -152,6 +152,23 @@ fn peer_pid(stream: &UnixStream) -> Option<u32> {
         return (ok == 0 && credentials.uid == libc::geteuid() && credentials.pid > 0)
             .then_some(credentials.pid as u32);
     }
+    #[cfg(target_os = "freebsd")]
+    unsafe {
+        let mut credentials: libc::xucred = std::mem::zeroed();
+        let mut size = std::mem::size_of::<libc::xucred>() as libc::socklen_t;
+        // 只信任内核提供的同用户身份，不采用消息里自报的 PID。
+        let ok = libc::getsockopt(
+            fd,
+            libc::SOL_LOCAL,
+            libc::LOCAL_PEERCRED,
+            (&mut credentials as *mut libc::xucred).cast(),
+            &mut size,
+        );
+        let pid = credentials.cr_pid__c_anonymous_union.cr_pid;
+        return (ok == 0 && credentials.cr_version == libc::XUCRED_VERSION
+            && credentials.cr_uid == libc::geteuid() && pid > 0)
+            .then_some(pid as u32);
+    }
     #[cfg(target_os = "macos")]
     unsafe {
         let (mut uid, mut gid) = (0, 0);
@@ -169,7 +186,7 @@ fn peer_pid(stream: &UnixStream) -> Option<u32> {
         );
         return (ok == 0 && pid > 0).then_some(pid as u32);
     }
-    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "macos")))]
     None
 }
 
