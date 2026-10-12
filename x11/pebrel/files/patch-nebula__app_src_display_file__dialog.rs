--- nebula_app/src/display/file_dialog.rs.orig	2026-10-11 12:00:27 UTC
+++ nebula_app/src/display/file_dialog.rs
@@ -3,7 +3,7 @@ use super::window::Window;
 #[cfg(feature = "legacy-shell")]
 use super::window::Window;
 
-#[cfg(any(target_os = "linux", target_os = "macos"))]
+#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
 #[path = "file_dialog/desktop.rs"]
 mod platform;
 #[cfg(windows)]
