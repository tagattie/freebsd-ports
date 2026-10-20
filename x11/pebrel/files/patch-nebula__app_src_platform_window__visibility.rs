--- nebula_app/src/platform/window_visibility.rs.orig	2026-10-16 12:40:11 UTC
+++ nebula_app/src/platform/window_visibility.rs
@@ -2,7 +2,7 @@ use winit::raw_window_handle::{HasWindowHandle, RawWin
 use gpui::Window;
 use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) fn is_wayland(window: &Window) -> bool {
     HasWindowHandle::window_handle(window)
         .is_ok_and(|handle| matches!(handle.as_raw(), RawWindowHandle::Wayland(_)))
@@ -23,7 +23,7 @@ pub(crate) fn set_visible(window: &Window, visible: bo
             }
             true
         },
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         RawWindowHandle::Xcb(_) | RawWindowHandle::Wayland(_) => {
             // 框架拥有首帧与事件队列；绕过它直接 map 会漏掉隐藏窗口的恢复绘制。
             window.set_window_visible(visible)
