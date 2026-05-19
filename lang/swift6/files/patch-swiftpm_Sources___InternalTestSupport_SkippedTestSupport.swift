--- swiftpm/Sources/_InternalTestSupport/SkippedTestSupport.swift.orig	2026-01-28 13:55:05 UTC
+++ swiftpm/Sources/_InternalTestSupport/SkippedTestSupport.swift
@@ -9,6 +9,8 @@
  See http://swift.org/CONTRIBUTORS.txt for Swift project authors
  */
 
+#if !os(FreeBSD)
+
 import class Foundation.FileManager
 import class Foundation.ProcessInfo
 import Basics
@@ -122,3 +124,5 @@ extension Tag.Feature.PackageType {
     @Tag public static var CommandPlugin: Tag
     @Tag public static var Macro: Tag
 }
+
+#endif
