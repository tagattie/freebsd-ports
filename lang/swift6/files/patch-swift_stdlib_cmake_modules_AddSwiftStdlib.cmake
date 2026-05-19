--- swift/stdlib/cmake/modules/AddSwiftStdlib.cmake.orig	2025-12-09 23:45:54 UTC
+++ swift/stdlib/cmake/modules/AddSwiftStdlib.cmake
@@ -1329,6 +1329,10 @@ function(add_swift_target_library_single target name)
     set_target_properties("${target}"
       PROPERTIES
       INSTALL_RPATH "$ORIGIN")
+  elseif("${SWIFTLIB_SINGLE_SDK}" STREQUAL "FREEBSD")
+    set_target_properties("${target}"
+      PROPERTIES
+      INSTALL_RPATH "$ORIGIN")
   endif()
 
   set_target_properties("${target}" PROPERTIES BUILD_WITH_INSTALL_RPATH YES)
