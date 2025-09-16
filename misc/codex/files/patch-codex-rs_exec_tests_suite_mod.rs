--- codex-rs/exec/tests/suite/mod.rs.orig	2025-09-15 17:04:50 UTC
+++ codex-rs/exec/tests/suite/mod.rs
@@ -2,4 +2,5 @@ mod resume;
 mod apply_patch;
 mod common;
 mod resume;
+#[cfg(not(target_os = "freebsd"))]
 mod sandbox;
