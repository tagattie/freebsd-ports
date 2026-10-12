--- mobile/link/src/bin/relay.rs.orig	2026-10-11 11:51:30 UTC
+++ mobile/link/src/bin/relay.rs
@@ -187,7 +187,14 @@ fn require_linux_root() -> io::Result<()> {
         }
         Err(io::Error::new(io::ErrorKind::PermissionDenied, "root_required"))
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(target_os = "freebsd")]
+    {
+        if unsafe { libc::geteuid() } == 0 {
+            return Ok(());
+        }
+        Err(io::Error::new(io::ErrorKind::PermissionDenied, "root_required"))
+    }
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     {
         Err(io::Error::other("linux_systemd_required"))
     }
