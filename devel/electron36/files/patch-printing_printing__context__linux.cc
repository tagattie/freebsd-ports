--- printing/printing_context_linux.cc.orig	2025-05-11 11:50:45 UTC
+++ printing/printing_context_linux.cc
@@ -23,7 +23,7 @@
 #endif
 
 // Avoid using LinuxUi on Fuchsia.
-#if BUILDFLAG(IS_LINUX)
+#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
 #include "ui/linux/linux_ui.h"
 #endif
 
@@ -65,7 +65,7 @@ mojom::ResultCode PrintingContextLinux::UseDefaultSett
 
   ResetSettings();
 
-#if BUILDFLAG(IS_LINUX)
+#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
   if (!ui::LinuxUi::instance())
     return mojom::ResultCode::kSuccess;
 
@@ -81,7 +81,7 @@ gfx::Size PrintingContextLinux::GetPdfPaperSizeDeviceU
 }
 
 gfx::Size PrintingContextLinux::GetPdfPaperSizeDeviceUnits() {
-#if BUILDFLAG(IS_LINUX)
+#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
   if (ui::LinuxUi::instance())
     return ui::LinuxUi::instance()->GetPdfPaperSize(this);
 #endif
@@ -94,7 +94,7 @@ mojom::ResultCode PrintingContextLinux::UpdatePrinterS
   DCHECK(!printer_settings.show_system_dialog);
   DCHECK(!in_print_job_);
 
-#if BUILDFLAG(IS_LINUX)
+#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_BSD)
   if (!ui::LinuxUi::instance())
     return mojom::ResultCode::kSuccess;
 
