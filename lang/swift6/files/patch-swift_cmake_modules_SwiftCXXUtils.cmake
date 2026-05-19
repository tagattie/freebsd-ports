--- swift/cmake/modules/SwiftCXXUtils.cmake.orig	2025-12-09 23:45:54 UTC
+++ swift/cmake/modules/SwiftCXXUtils.cmake
@@ -1,6 +1,5 @@ set(SWIFT_LIBSTDCXX_PLATFORMS
 # Platforms that use libstdc++ as the system-wide default C++ standard library.
 set(SWIFT_LIBSTDCXX_PLATFORMS
     "LINUX"
-    "FREEBSD"
     "CYGWIN"
     "HAIKU")
