--- nebula_app/src/gpui_shell/workspace/quick_terminal.rs.orig	2026-10-16 12:40:11 UTC
+++ nebula_app/src/gpui_shell/workspace/quick_terminal.rs
@@ -41,15 +41,15 @@ impl Global for QuickTerminalHotkey {}
 
 impl Global for QuickTerminalHotkey {}
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 struct PortalHotkey(crate::platform::global_shortcut::Portal, Option<String>);
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl Global for PortalHotkey {}
 
 impl NebulaWorkspace {
     /// 注册快速终端热键并启动事件泵。只应由初始窗口调用一次。
     pub(super) fn start_quick_terminal_hotkey(window: &Window, cx: &mut Context<Self>) {
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if crate::platform::window_visibility::is_wayland(window) {
             Self::start_portal_hotkey(cx);
             return;
@@ -93,7 +93,7 @@ impl NebulaWorkspace {
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl NebulaWorkspace {
     fn start_portal_hotkey(cx: &mut Context<Self>) {
         if cx.has_global::<PortalHotkey>() {
