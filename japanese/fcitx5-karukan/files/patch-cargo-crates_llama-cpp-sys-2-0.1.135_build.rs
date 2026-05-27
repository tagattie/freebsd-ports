--- cargo-crates/llama-cpp-sys-2-0.1.135/build.rs.orig	2006-07-24 01:21:28 UTC
+++ cargo-crates/llama-cpp-sys-2-0.1.135/build.rs
@@ -20,6 +20,7 @@ enum TargetOs {
     Apple(AppleVariant),
     Linux,
     Android,
+    FreeBSD,
 }
 
 macro_rules! debug_log {
@@ -55,6 +56,8 @@ fn parse_target_os() -> Result<(TargetOs, String), Str
         Ok((TargetOs::Android, target))
     } else if target.contains("linux") {
         Ok((TargetOs::Linux, target))
+    } else if target.contains("freebsd") {
+        Ok((TargetOs::FreeBSD, target))
     } else {
         Err(target)
     }
@@ -761,7 +764,7 @@ fn main() {
                 config.cflag("/FS");
                 config.cxxflag("/FS");
             }
-            TargetOs::Linux => {
+            TargetOs::Linux | TargetOs::FreeBSD => {
                 // If we are not using system provided vulkan SDK, add vulkan libs for linking
                 if let Ok(vulkan_path) = env::var("VULKAN_SDK") {
                     let vulkan_lib_path = Path::new(&vulkan_path).join("lib");
@@ -968,6 +971,9 @@ fn main() {
         }
         TargetOs::Linux => {
             println!("cargo:rustc-link-lib=dylib=stdc++");
+        }
+        TargetOs::FreeBSD => {
+            println!("cargo:rustc-link-lib=dylib=c++");
         }
         TargetOs::Apple(variant) => {
             println!("cargo:rustc-link-lib=framework=Foundation");
