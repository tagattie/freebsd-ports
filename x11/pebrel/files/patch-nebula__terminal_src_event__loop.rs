--- nebula_terminal/src/event_loop.rs.orig	2026-10-16 12:40:11 UTC
+++ nebula_terminal/src/event_loop.rs
@@ -737,7 +737,7 @@ where
                                     // just loop back round for the inevitable `Exited` event.
                                     // This sucks, but checking the process is either racy or
                                     // blocking.
-                                    #[cfg(target_os = "linux")]
+                                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                                     if err.raw_os_error() == Some(libc::EIO) {
                                         continue;
                                     }
