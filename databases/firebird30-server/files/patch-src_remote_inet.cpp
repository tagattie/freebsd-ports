--- src/remote/inet.cpp.orig	2024-08-08 09:26:45 UTC
+++ src/remote/inet.cpp
@@ -960,7 +960,7 @@ rem_port* INET_connect(const TEXT* name,
 		gai_hints.ai_family = ((host.hasData() || !ipv6) ? AF_UNSPEC : AF_INET6);
 	gai_hints.ai_socktype = SOCK_STREAM;
 
-#if !defined(WIN_NT) && !defined(__clang__)
+#if !defined(WIN_NT) && !defined(__FreeBSD__) && !defined(__DragonFly__)
 	gai_hints.ai_protocol = SOL_TCP;
 #else
 	gai_hints.ai_protocol = IPPROTO_TCP;
@@ -1175,6 +1175,12 @@ static rem_port* listener_socket(rem_port* port, USHOR
 	setFastLoopbackOption(port);
 
 	inet_ports->registerPort(port);
+
+	char *parent_pid;
+	if (parent_pid = getenv("FB_SIGNAL_PROCESS"))
+	{
+		kill(atoi(parent_pid), SIGUSR1);
+	}
 
 	if (flag & SRVR_multi_client)
 	{
