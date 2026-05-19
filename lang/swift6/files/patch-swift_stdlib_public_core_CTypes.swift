--- swift/stdlib/public/core/CTypes.swift.orig	2025-12-09 23:45:54 UTC
+++ swift/stdlib/public/core/CTypes.swift
@@ -116,6 +116,9 @@ public typealias CLongDouble = Float80
 #elseif os(FreeBSD)
 #if arch(x86_64) || arch(i386)
 public typealias CLongDouble = Float80
+#elseif arch(arm64)
+// FreeBSD on arm64 uses 128-bit floating point type for 'long double'.
+// Swift doesn't have Float128 yet so we don't define CLongDouble.
 #else
 #error("CLongDouble needs to be defined for this FreeBSD architecture")
 #endif
