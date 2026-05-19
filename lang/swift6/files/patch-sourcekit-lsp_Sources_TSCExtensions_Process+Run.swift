--- sourcekit-lsp/Sources/TSCExtensions/Process+Run.swift.orig	2026-01-28 22:16:35 UTC
+++ sourcekit-lsp/Sources/TSCExtensions/Process+Run.swift
@@ -184,6 +184,8 @@ private func setProcessPriority(pid: Process.ProcessID
   // can only be elevated, this means that we can effectively only change a process's priority once, when it is created.
   // All subsequent calls to `setpriority` will fail. Because of this, don't log an error.
   setpriority(PRIO_PROCESS, UInt32(pid), newPriority.posixProcessPriority)
+  #elseif os(FreeBSD)
+  setpriority(PRIO_PROCESS, Int32(pid), newPriority.posixProcessPriority)
   #else
   setpriority(__priority_which_t(PRIO_PROCESS.rawValue), UInt32(pid), newPriority.posixProcessPriority)
   #endif
