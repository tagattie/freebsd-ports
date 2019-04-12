--- CMakeModules/CompilerFlags.cmake.orig	2019-01-09 12:42:39 UTC
+++ CMakeModules/CompilerFlags.cmake
@@ -14,8 +14,8 @@ enable_if_supported(COMPILER_FLAGS_THIRD_PARTY "-Wno-s
 enable_if_supported(COMPILER_FLAGS_THIRD_PARTY "/wd4244")
 enable_if_supported(COMPILER_FLAGS_THIRD_PARTY "/wd4267")
 
-enable_if_links(LINK_FLAGS_RELEASE "-flto")
-enable_if_links(LINK_FLAGS "-fuse-ld=gold")
+# enable_if_links(LINK_FLAGS_RELEASE "-flto")
+# enable_if_links(LINK_FLAGS "-fuse-ld=gold")
 
 set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${COMPILER_FLAGS}")
 set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${COMPILER_FLAGS}")
