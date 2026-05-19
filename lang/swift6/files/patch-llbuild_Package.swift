--- llbuild/Package.swift.orig	2025-11-14 16:33:33 UTC
+++ llbuild/Package.swift
@@ -97,6 +97,13 @@ let package = Package(
             exclude: ["CMakeLists.txt"]
         ),
 
+        // MARK: sqlite3 as a system library
+        .systemLibrary(
+            name: "LLBSQLite3",
+            path: "lib/LLBSQLite3",
+            pkgConfig: "sqlite3"
+        ),
+
         // MARK: Components
 
         .target(
@@ -262,6 +269,7 @@ let package = Package(
             path: "lib/llvm/Support",
             linkerSettings: [
                 .linkedLibrary("m", .when(platforms: [.linux])),
+                .linkedLibrary("execinfo"),
             ] + terminfoLibraries
         ),
     ],
