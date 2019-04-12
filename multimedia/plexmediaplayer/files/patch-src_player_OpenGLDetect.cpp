--- src/player/OpenGLDetect.cpp.orig	2019-01-09 12:42:39 UTC
+++ src/player/OpenGLDetect.cpp
@@ -30,7 +30,7 @@ void detectOpenGLLate()
 {
 }
 
-#elif defined(Q_OS_LINUX)
+#elif defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
 
 ///////////////////////////////////////////////////////////////////////////////////////////////////
 // Attempt to reuse mpv's code for detecting whether we want GLX or EGL (which
