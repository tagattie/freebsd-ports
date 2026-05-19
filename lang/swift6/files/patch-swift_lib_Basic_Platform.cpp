--- swift/lib/Basic/Platform.cpp.orig	2026-01-30 13:02:22 UTC
+++ swift/lib/Basic/Platform.cpp
@@ -435,7 +435,9 @@ llvm::Triple swift::getTargetSpecificModuleTriple(cons
   }
 
   if (triple.isOSFreeBSD()) {
-    return swift::getUnversionedTriple(triple);
+    // return swift::getUnversionedTriple(triple);
+    // just return triple same as other platforms
+    return triple;
   }
 
   if (triple.isOSOpenBSD()) {
