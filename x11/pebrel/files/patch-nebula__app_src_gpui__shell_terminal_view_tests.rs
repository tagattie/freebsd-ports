--- nebula_app/src/gpui_shell/terminal/view/tests.rs.orig	2026-10-11 12:01:43 UTC
+++ nebula_app/src/gpui_shell/terminal/view/tests.rs
@@ -109,7 +109,7 @@ fn all_font_faces_explicitly_control_ligatures_without
 
 // Linux's headless platform uses the real text shaper with no display or GPU.
 // GPUI's normal test context uses NoopTextSystem and cannot prove ligatures.
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 #[test]
 fn real_bundled_font_shapes_ligatures_and_preserves_the_cached_natural_layout() {
     use crate::gpui_shell::terminal::ligatures::shape_ascii_span;
