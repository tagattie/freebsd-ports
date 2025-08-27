--- src/window.cpp.orig	2025-08-27 08:21:01 UTC
+++ src/window.cpp
@@ -13,7 +13,20 @@
 #include <unistd.h>
 #include <wayland-client-protocol.h>
 
+#if defined(__FreeBSD__)
+#include <sys/ucred.h>
+#include <sys/un.h>
 static pid_t pid_from_fd(const int fd) {
+    xucred cred{};
+    socklen_t len = sizeof(struct xucred);
+    if (getsockopt(fd, SOL_LOCAL, LOCAL_PEERCRED, &cred, &len) == -1) {
+        perror("getsockopt failed");
+        exit(1);
+    }
+    return cred.cr_pid;
+}
+#else
+static pid_t pid_from_fd(const int fd) {
     ucred cred{};
     socklen_t len = sizeof(struct ucred);
     if (getsockopt(fd, SOL_SOCKET, SO_PEERCRED, &cred, &len) == -1) {
@@ -22,6 +35,7 @@ static pid_t pid_from_fd(const int fd) {
     }
     return cred.pid;
 }
+#endif
 
 static std::string process_name_from_pid(const pid_t pid) {
     const std::string procpath = QString::asprintf("/proc/%d/comm", pid).toStdString();
