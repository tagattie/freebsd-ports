--- apps/cli/src/services/cli-platform-utils.service.ts.orig	2023-04-26 11:30:57 UTC
+++ apps/cli/src/services/cli-platform-utils.service.ts
@@ -76,7 +76,7 @@ export class CliPlatformUtilsService implements Platfo
   }
 
   launchUri(uri: string, options?: any): void {
-    if (process.platform === "linux") {
+    if (process.platform === "linux" || process.platform === "freebsd") {
       child_process.spawnSync("xdg-open", [uri]);
     } else {
       open(uri);
