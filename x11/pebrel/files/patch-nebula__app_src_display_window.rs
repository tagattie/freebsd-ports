--- nebula_app/src/display/window.rs.orig	2026-10-11 12:01:10 UTC
+++ nebula_app/src/display/window.rs
@@ -312,7 +312,7 @@ impl Window {
     }
 
     #[inline]
-    #[cfg(any(target_os = "linux", target_os = "macos"))]
+    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
     pub(super) fn native_window(&self) -> &WinitWindow {
         &self.window
     }
