--- swift/lib/Driver/ToolChains.cpp.orig	2026-01-30 21:21:45 UTC
+++ swift/lib/Driver/ToolChains.cpp
@@ -1600,6 +1600,9 @@ void ToolChain::getResourceDirPath(SmallVectorImpl<cha
   StringRef libSubDir = getPlatformNameForTriple(getTriple());
   if (tripleIsMacCatalystEnvironment(getTriple()))
     libSubDir = "maccatalyst";
+  if (getTriple().isOSFreeBSD()) {
+    libSubDir = "freebsd";
+  }
   llvm::sys::path::append(resourceDirPath, libSubDir);
 }
 
