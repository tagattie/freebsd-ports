--- ui/views/widget/root_view.cc.orig	2024-12-22 12:24:29 UTC
+++ ui/views/widget/root_view.cc
@@ -110,7 +110,7 @@ class AnnounceTextView : public View {
 #if BUILDFLAG(IS_CHROMEOS)
     // On ChromeOS, kAlert role can invoke an unnecessary event on reparenting.
     GetViewAccessibility().SetRole(ax::mojom::Role::kStaticText);
-#elif BUILDFLAG(IS_LINUX)
+#elif BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
     // TODO(crbug.com/40658933): Use live regions (do not use alerts).
     // May require setting kLiveStatus, kContainerLiveStatus to "polite".
     GetViewAccessibility().SetRole(ax::mojom::Role::kAlert);
