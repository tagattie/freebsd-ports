--- swift/lib/IRGen/IRGen.cpp.orig	2025-12-09 23:45:54 UTC
+++ swift/lib/IRGen/IRGen.cpp
@@ -158,6 +158,9 @@ swift::getIRTargetOptions(const IRGenOptions &Opts, AS
   if (Clang->getTargetInfo().getTriple().isOSBinFormatWasm())
     TargetOpts.ThreadModel = llvm::ThreadModel::Single;
 
+  if (Clang->getTargetInfo().getTriple().isOSFreeBSD())
+    TargetOpts.UseInitArray = 1;
+
   if (Opts.EnableGlobalISel) {
     TargetOpts.EnableGlobalISel = true;
     TargetOpts.GlobalISelAbort = GlobalISelAbortMode::DisableWithDiag;
