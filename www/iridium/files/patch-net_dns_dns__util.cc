--- net/dns/dns_util.cc.orig	2025-05-07 06:48:23 UTC
+++ net/dns/dns_util.cc
@@ -29,6 +29,8 @@
 #include "net/dns/public/util.h"
 #include "net/third_party/uri_template/uri_template.h"
 
+#include <sys/socket.h>
+
 #if BUILDFLAG(IS_POSIX)
 #include <net/if.h>
 #include <netinet/in.h>
