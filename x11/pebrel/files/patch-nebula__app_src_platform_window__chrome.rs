--- nebula_app/src/platform/window_chrome.rs.orig	2026-10-20 11:48:25 UTC
+++ nebula_app/src/platform/window_chrome.rs
@@ -8,7 +8,7 @@ pub(crate) fn application_id(_fallback: &str) -> Strin
 mod macos;
 
 pub(crate) fn application_id(_fallback: &str) -> String {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use std::sync::OnceLock;
         static ID: OnceLock<&'static str> = OnceLock::new();
@@ -28,13 +28,13 @@ pub(crate) fn application_id(_fallback: &str) -> Strin
         })
         .to_string()
     }
-    #[cfg(not(target_os = "linux"))]
+    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
     _fallback.to_owned()
 }
 
 #[cfg(test)]
 pub(crate) fn native_test_window_id(_window: &Window) -> Option<(&'static str, u32)> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
         match HasWindowHandle::window_handle(_window).ok()?.as_raw() {
