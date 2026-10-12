--- nebula_app/src/platform/file_manager.rs.orig	2026-10-11 12:11:27 UTC
+++ nebula_app/src/platform/file_manager.rs
@@ -5,7 +5,7 @@ pub fn open(path: &Path) -> std::io::Result<()> {
 
 pub fn open(path: &Path) -> std::io::Result<()> {
     let res = open_command(Platform::current(), path).spawn().map(|_| ());
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     if res.is_err() {
         if command("gio").arg("open").arg(path).spawn().is_ok() {
             return Ok(());
