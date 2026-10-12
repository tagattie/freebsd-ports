--- nebula_app/src/platform/sound.rs.orig	2026-10-11 12:13:30 UTC
+++ nebula_app/src/platform/sound.rs
@@ -8,9 +8,9 @@ pub(super) fn play() -> bool {
 
 pub(super) fn play() -> bool {
     let program = if cfg!(target_os = "macos") {
-        ["/usr/bin/afplay", "", ""]
+        ["/usr/bin/afplay", "", "", "", ""]
     } else {
-        ["/usr/bin/pw-play", "/usr/bin/paplay", "/bin/paplay"]
+        ["/usr/local/bin/paplay", "/usr/local/bin/pw-play", "/usr/bin/pw-play", "/usr/bin/paplay", "/bin/paplay"]
     }
     .into_iter()
     .find(|path| !path.is_empty() && std::path::Path::new(path).is_file());
@@ -37,7 +37,7 @@ pub(super) fn play() -> bool {
         else {
             return;
         };
-        if cfg!(target_os = "linux")
+        if cfg!(any(target_os = "linux", target_os = "freebsd"))
             && let Some(mut input) = child.stdin.take()
         {
             // 80ms 衰减正弦，避免依赖桌面主题是否安装了 bell.oga。
