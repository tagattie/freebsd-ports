--- src/ui/KonvergoWindow.h.orig	2019-04-11 12:29:26 UTC
+++ src/ui/KonvergoWindow.h
@@ -50,7 +50,7 @@ class KonvergoWindow : public QQuickWindow (public)
   bool isAlwaysOnTop()
   {
     Qt::WindowFlags forceOnTopFlags = Qt::WindowStaysOnTopHint;
-#ifdef Q_OS_LINUX
+#if defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
     forceOnTopFlags = forceOnTopFlags | Qt::X11BypassWindowManagerHint;
 #endif
     return (flags() & forceOnTopFlags);
