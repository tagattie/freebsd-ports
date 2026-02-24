--- swift/cmake/modules/SwiftConfigureSDK.cmake.orig	2025-12-09 23:45:54 UTC
+++ swift/cmake/modules/SwiftConfigureSDK.cmake
@@ -421,7 +421,7 @@ macro(configure_sdk_unix name architectures)
           message(FATAL_ERROR "unknown arch for ${prefix}: ${arch}")
         endif()
       elseif("${prefix}" STREQUAL "FREEBSD")
-        if(NOT arch MATCHES "(arm64|x86_64)")
+        if(NOT arch MATCHES "(aarch64|x86_64)")
           message(FATAL_ERROR "unsupported arch for FreeBSD: ${arch}")
         endif()
 
@@ -432,7 +432,7 @@ macro(configure_sdk_unix name architectures)
         string(REGEX REPLACE "[-].*" "" freebsd_system_version ${CMAKE_SYSTEM_VERSION})
         message(STATUS "FreeBSD Version: ${freebsd_system_version}")
 
-        set(SWIFT_SDK_FREEBSD_ARCH_${arch}_TRIPLE "${arch}-unknown-freebsd")
+        set(SWIFT_SDK_FREEBSD_ARCH_${arch}_TRIPLE "${arch}-unknown-freebsd${freebsd_system_version}")
       elseif("${prefix}" STREQUAL "OPENBSD")
         if(NOT arch STREQUAL "x86_64" AND NOT arch STREQUAL "aarch64")
           message(FATAL_ERROR "unsupported arch for OpenBSD: ${arch}")
