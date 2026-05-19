--- swift-corelibs-libdispatch/src/io.c.orig	2026-01-23 20:31:17 UTC
+++ swift-corelibs-libdispatch/src/io.c
@@ -2310,7 +2310,7 @@ _dispatch_operation_advise(dispatch_operation_t op, si
 _dispatch_operation_advise(dispatch_operation_t op, size_t chunk_size)
 {
 	_dispatch_op_debug("advise", op);
-#if defined(_WIN32)
+#if defined(_WIN32) || defined(__FreeBSD__)
 	(void)op;
 	(void)chunk_size;
 #else
