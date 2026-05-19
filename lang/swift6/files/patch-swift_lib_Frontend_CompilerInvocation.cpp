--- swift/lib/Frontend/CompilerInvocation.cpp.orig	2025-12-09 23:45:54 UTC
+++ swift/lib/Frontend/CompilerInvocation.cpp
@@ -220,6 +220,8 @@ static void updateRuntimeLibraryPaths(SearchPathOption
     LibSubDir = "maccatalyst";
   if (LangOpts.hasFeature(Feature::Embedded))
     LibSubDir = "embedded";
+  if (Triple.isOSFreeBSD())
+    LibSubDir = "freebsd";
 
   SearchPathOpts.RuntimeLibraryPaths.clear();
 
@@ -287,7 +289,10 @@ static void updateRuntimeLibraryPaths(SearchPathOption
     if (!Triple.isOSDarwin()) {
       // Use the non-architecture suffixed form with directory-layout
       // swiftmodules.
-      llvm::sys::path::append(LibPath, getPlatformNameForTriple(Triple));
+      if (Triple.isOSFreeBSD())
+        llvm::sys::path::append(LibPath, LibSubDir);
+      else
+        llvm::sys::path::append(LibPath, getPlatformNameForTriple(Triple));
       RuntimeLibraryImportPaths.push_back(std::string(LibPath.str()));
 
       // Compatibility with older releases - use the architecture suffixed form
