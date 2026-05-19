--- swiftpm/Sources/_IntegrationTestSupport/Helpers.swift.orig	2026-01-28 13:54:21 UTC
+++ swiftpm/Sources/_IntegrationTestSupport/Helpers.swift
@@ -8,6 +8,8 @@
  See http://swift.org/CONTRIBUTORS.txt for Swift project authors
  */
 
+#if !os(FreeBSD)
+
 import Foundation
 import Testing
 import _InternalTestSupport
@@ -188,4 +190,6 @@ extension AsyncProcessResult {
         \((try? utf8stderrOutput()) ?? "")
         """
     }
-}
\ No newline at end of file
+}
+
+#endif
