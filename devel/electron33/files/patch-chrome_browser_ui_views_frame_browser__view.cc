--- chrome/browser/ui/views/frame/browser_view.cc.orig	2024-10-16 21:31:02 UTC
+++ chrome/browser/ui/views/frame/browser_view.cc
@@ -2346,7 +2346,7 @@ void BrowserView::TabDraggingStatusChanged(bool is_dra
   // CrOS cleanup is done.
 // TODO(crbug.com/40118868): Revisit the macro expression once build flag switch
 // of lacros-chrome is complete.
-#if !(BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS_LACROS))
+#if !(BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS_LACROS) || BUILDFLAG(IS_BSD))
   contents_web_view_->SetFastResize(is_dragging);
   if (!is_dragging) {
     // When tab dragging is ended, we need to make sure the web contents get
