--- nebula_app/src/platform/window_visibility.rs.orig	2026-10-11 12:16:56 UTC
+++ nebula_app/src/platform/window_visibility.rs
@@ -2,7 +2,7 @@ use winit::raw_window_handle::{HasWindowHandle, RawWin
 use gpui::Window;
 use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) fn is_wayland(window: &Window) -> bool {
     HasWindowHandle::window_handle(window)
         .is_ok_and(|handle| matches!(handle.as_raw(), RawWindowHandle::Wayland(_)))
@@ -23,9 +23,9 @@ pub(crate) fn set_visible(window: &Window, visible: bo
             }
             true
         },
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         RawWindowHandle::Xcb(handle) => xcb_visible(window, handle.window.get(), visible),
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         RawWindowHandle::Wayland(_) => {
             // xdg-shell 不提供任意隐藏或定位；让合成器管理最小化，避免破坏 surface 生命周期。
             if visible {
@@ -39,7 +39,7 @@ pub(crate) fn set_visible(window: &Window, visible: bo
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn xcb_visible(window: &Window, xid: u32, visible: bool) -> bool {
     use winit::raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
     #[repr(C)]
