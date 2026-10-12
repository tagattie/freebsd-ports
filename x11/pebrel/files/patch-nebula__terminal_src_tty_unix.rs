--- nebula_terminal/src/tty/unix.rs.orig	2026-10-07 15:23:24 UTC
+++ nebula_terminal/src/tty/unix.rs
@@ -20,7 +20,7 @@ use rustix_openpty::rustix::termios::Winsize;
 use polling::{Event, PollMode, Poller};
 use rustix_openpty::openpty;
 use rustix_openpty::rustix::termios::Winsize;
-#[cfg(any(target_os = "linux", target_os = "macos"))]
+#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "macos"))]
 use rustix_openpty::rustix::termios::{self, InputModes, OptionalActions};
 use signal_hook::low_level::{pipe as signal_pipe, unregister as unregister_signal};
 use signal_hook::{SigId, consts as sigconsts};
