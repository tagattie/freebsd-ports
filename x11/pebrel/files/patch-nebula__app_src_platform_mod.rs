--- nebula_app/src/platform/mod.rs.orig	2026-10-11 12:12:10 UTC
+++ nebula_app/src/platform/mod.rs
@@ -29,7 +29,7 @@ pub mod fonts;
 pub(crate) mod file_preview;
 #[cfg(feature = "gpui-shell")]
 pub mod fonts;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) mod global_shortcut;
 #[cfg(all(windows, feature = "gpui-shell"))]
 pub(crate) mod keyboard;
