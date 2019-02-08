--- setup.py.orig	2019-02-08 04:52:49 UTC
+++ setup.py
@@ -8,6 +8,7 @@ import subprocess
 
 ROOT_DIR = dirname(abspath(__file__))
 SHARE_PATH = join(ROOT_DIR, "share")
+MAN_PATH = join(ROOT_DIR, "man")
 PO_DIR = 'translations'
 VERSION = open(join(ROOT_DIR, "VERSION")).read().strip()
 
@@ -20,8 +21,13 @@ blacklist = ['index.theme']
 
 data_files = []
 for path, dirs, files in os.walk(SHARE_PATH):
-    data_files.append(tuple((path.replace(SHARE_PATH,"share", 1),
-        [join(path, file) for file in files if file not in blacklist])))
+    tl = [join(path, file) for file in files if file not in blacklist]
+    if tl != []:
+        data_files.append(tuple((path.replace(SHARE_PATH,"share", 1), tl)))
+for path, dirs, files in os.walk(MAN_PATH):
+    if files != []:
+        data_files.append(tuple((path.replace(MAN_PATH,"man", 1),
+            [join(path, file) for file in files])))
 
 desktop_name = "caffeine.desktop"
 desktop_file = join("share", "applications", desktop_name)
