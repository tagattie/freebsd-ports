--- net/base/features.cc.orig	2025-03-24 20:50:14 UTC
+++ net/base/features.cc
@@ -26,7 +26,7 @@ BASE_FEATURE(kAsyncDns,
 BASE_FEATURE(kAsyncDns,
              "AsyncDns",
 #if BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_MAC) || BUILDFLAG(IS_ANDROID) || \
-    BUILDFLAG(IS_WIN) || BUILDFLAG(IS_LINUX)
+    BUILDFLAG(IS_WIN) || BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
              base::FEATURE_ENABLED_BY_DEFAULT
 #else
              base::FEATURE_DISABLED_BY_DEFAULT
@@ -549,7 +549,12 @@ BASE_FEATURE(kSpdyHeadersToHttpResponseUseBuilder,
              "SpdyHeadersToHttpResponseUseBuilder",
              base::FEATURE_DISABLED_BY_DEFAULT);
 
+#if BUILDFLAG(IS_OPENBSD)
+// No IP_RECVTOS support
+BASE_FEATURE(kReportEcn, "ReportEcn", base::FEATURE_DISABLED_BY_DEFAULT);
+#else
 BASE_FEATURE(kReportEcn, "ReportEcn", base::FEATURE_ENABLED_BY_DEFAULT);
+#endif
 
 BASE_FEATURE(kUseNewAlpsCodepointHttp2,
              "UseNewAlpsCodepointHttp2",
