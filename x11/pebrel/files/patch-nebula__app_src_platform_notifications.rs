--- nebula_app/src/platform/notifications.rs.orig	2026-10-11 12:12:45 UTC
+++ nebula_app/src/platform/notifications.rs
@@ -82,7 +82,7 @@ pub(crate) fn toast_actionable(
     windows::enqueue(title, body, activation, actions);
 }
 
-#[cfg(any(target_os = "linux", target_os = "macos"))]
+#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
 pub(crate) fn toast_actionable(
     title: &str,
     body: &str,
@@ -132,7 +132,7 @@ pub(crate) fn toast_actionable(
     }
 }
 
-#[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
+#[cfg(not(any(windows, target_os = "linux", target_os = "freebsd", target_os = "macos")))]
 pub(crate) fn toast_actionable(
     title: &str,
     body: &str,
