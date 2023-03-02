--- apps/cli/src/services/cli-platform-utils.service.ts.orig	2023-02-28 10:48:52 UTC
+++ apps/cli/src/services/cli-platform-utils.service.ts
@@ -77,7 +77,7 @@ export class CliPlatformUtilsService implements Platfo
   }
 
   launchUri(uri: string, options?: any): void {
-    if (process.platform === "linux") {
+    if (process.platform === "linux" || process.platform === "freebsd") {
       child_process.spawnSync("xdg-open", [uri]);
     } else {
       open(uri);
