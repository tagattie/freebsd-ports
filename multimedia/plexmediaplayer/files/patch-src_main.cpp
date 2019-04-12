--- src/main.cpp.orig	2019-01-09 12:42:39 UTC
+++ src/main.cpp
@@ -34,7 +34,7 @@
 #include "PFMoveApplication.h"
 #endif
 
-#if defined(Q_OS_MAC) || defined(Q_OS_LINUX)
+#if defined(Q_OS_MAC) || defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
 #include "SignalManager.h"
 #endif
 
@@ -158,7 +158,7 @@ int main(int argc, char *argv[])
       qputenv("QT_SCALE_FACTOR", scale.toUtf8());
 
     QApplication app(newArgc, newArgv);
-#if defined(Q_OS_WIN) || defined(Q_OS_LINUX)
+#if defined(Q_OS_WIN) || defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)
     // Setting window icon on OSX will break user ability to change it
     app.setWindowIcon(QIcon(":/images/icon.png"));
 #endif
