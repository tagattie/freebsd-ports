--- CMakeModules/FetchDependencies.cmake.orig	2019-01-09 12:42:39 UTC
+++ CMakeModules/FetchDependencies.cmake
@@ -141,7 +141,7 @@ function(download_deps DD_NAME)
 
   set(${DD_DIRECTORY} ${DEP_DIR}/${DEP_DIRNAME} PARENT_SCOPE)
 
-  get_content_of_url(URL ${CONTENT_HASH_URL} CONTENT_VAR CONTENT_HASH)
+#   get_content_of_url(URL ${CONTENT_HASH_URL} CONTENT_VAR CONTENT_HASH)
 
   string(SUBSTRING "${CONTENT_HASH}" 0 40 CONTENT_HASH)
 
