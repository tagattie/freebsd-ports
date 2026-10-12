--- nebula_app/src/platform/tray_native.rs.orig	2026-10-11 12:15:36 UTC
+++ nebula_app/src/platform/tray_native.rs
@@ -54,7 +54,7 @@ pub fn init_gpui(on_command: impl Fn(GpuiTrayCommand) 
             callback(GpuiTrayCommand::Focus(None));
         }
     }));
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     worker::start();
 }
 
@@ -94,7 +94,7 @@ fn refresh() {
     }
     #[cfg(target_os = "macos")]
     apply();
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     worker::refresh();
 }
 
@@ -175,14 +175,14 @@ pub fn shutdown() {
 }
 
 pub fn shutdown() {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     worker::shutdown();
     #[cfg(target_os = "macos")]
     NATIVE.with_borrow_mut(|native| *native = None);
     *ACTIONS.lock().unwrap_or_else(|e| e.into_inner()) = None;
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod worker {
     use super::*;
     use std::sync::mpsc;
