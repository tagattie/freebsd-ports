--- ui/gfx/switches.h.orig	2025-05-07 06:48:23 UTC
+++ ui/gfx/switches.h
@@ -18,7 +18,7 @@ GFX_SWITCHES_EXPORT extern const char kForcePrefersRed
 GFX_SWITCHES_EXPORT extern const char kForcePrefersNoReducedMotion[];
 GFX_SWITCHES_EXPORT extern const char kHeadless[];
 
-#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
+#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_BSD)
 GFX_SWITCHES_EXPORT extern const char kX11Display[];
 GFX_SWITCHES_EXPORT extern const char kNoXshm[];
 #endif
