--- src/app/widgets/preferences_dialog.rs.orig	2025-09-13 05:15:46 UTC
+++ src/app/widgets/preferences_dialog.rs
@@ -85,7 +85,7 @@ mod imp {
                     None,
                     "{n}/{total} collection enabled",
                     "{n}/{total} collections enabled",
-                    u64::try_from(n_enabled).unwrap(),
+                    u64::try_from(n_enabled).unwrap().try_into().unwrap(),
                 ),
                 n = n_enabled,
                 total = stalenhag::COLLECTIONS.len()
