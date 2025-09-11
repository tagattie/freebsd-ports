--- codex-rs/exec/tests/suite/mod.rs.orig	2025-09-11 03:59:06 UTC
+++ codex-rs/exec/tests/suite/mod.rs
@@ -1,4 +1,5 @@ mod common;
 // Aggregates all former standalone integration tests as modules.
 mod apply_patch;
 mod common;
+#[cfg(not(target_os = "freebsd"))]
 mod sandbox;
