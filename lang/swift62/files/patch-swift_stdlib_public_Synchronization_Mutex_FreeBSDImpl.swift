--- swift/stdlib/public/Synchronization/Mutex/FreeBSDImpl.swift.orig	2026-01-28 08:09:40 UTC
+++ swift/stdlib/public/Synchronization/Mutex/FreeBSDImpl.swift
@@ -0,0 +1,46 @@
+import Glibc
+
+@available(SwiftStdlib 6.0, *)
+@frozen
+@_staticExclusiveOnly
+@safe
+public struct _MutexHandle: ~Copyable {
+  @usableFromInline
+  let value: _Cell<pthread_mutex_t?>
+
+  @available(SwiftStdlib 6.0, *)
+  @_alwaysEmitIntoClient
+  @_transparent
+  public init() {
+    unsafe value = _Cell(nil)
+    _ = unsafe pthread_mutex_init(value._address, nil)
+  }
+
+  @available(SwiftStdlib 6.0, *)
+  @_alwaysEmitIntoClient
+  @_transparent
+  deinit {
+    _ = unsafe pthread_mutex_destroy(value._address)
+  }
+
+  @available(SwiftStdlib 6.0, *)
+  @_alwaysEmitIntoClient
+  @_transparent
+  internal borrowing func _lock() {
+    _ = unsafe pthread_mutex_lock(value._address)
+  }
+
+  @available(SwiftStdlib 6.0, *)
+  @_alwaysEmitIntoClient
+  @_transparent
+  internal borrowing func _tryLock() -> Bool {
+    return (unsafe pthread_mutex_trylock(value._address)) == 0
+  }
+
+  @available(SwiftStdlib 6.0, *)
+  @_alwaysEmitIntoClient
+  @_transparent
+  internal borrowing func _unlock() {
+    _ = unsafe pthread_mutex_unlock(value._address)
+  }
+}
