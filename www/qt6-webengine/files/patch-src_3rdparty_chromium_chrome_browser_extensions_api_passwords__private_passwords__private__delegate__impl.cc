--- src/3rdparty/chromium/chrome/browser/extensions/api/passwords_private/passwords_private_delegate_impl.cc.orig	2024-02-23 21:04:38 UTC
+++ src/3rdparty/chromium/chrome/browser/extensions/api/passwords_private/passwords_private_delegate_impl.cc
@@ -600,7 +600,7 @@ void PasswordsPrivateDelegateImpl::OnFetchingFamilyMem
 }
 
 void PasswordsPrivateDelegateImpl::OsReauthTimeoutCall() {
-#if !BUILDFLAG(IS_LINUX)
+#if !BUILDFLAG(IS_LINUX) && !BUILDFLAG(IS_BSD)
   PasswordsPrivateEventRouter* router =
       PasswordsPrivateEventRouterFactory::GetForProfile(profile_);
   if (router) {
