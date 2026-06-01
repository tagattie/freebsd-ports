--- karukan-im/build.rs.orig	2026-06-01 07:21:40 UTC
+++ karukan-im/build.rs
@@ -0,0 +1,3 @@
+fn main() {
+    println!("cargo:rustc-link-lib=dylib=omp");
+}
