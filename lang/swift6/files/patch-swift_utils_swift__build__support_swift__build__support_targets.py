--- swift/utils/swift_build_support/swift_build_support/targets.py.orig	2025-12-09 23:45:54 UTC
+++ swift/utils/swift_build_support/swift_build_support/targets.py
@@ -294,7 +294,7 @@ class StdlibDeploymentTarget(object):
         "riscv64",
         "s390x"])
 
-    FreeBSD = Platform("freebsd", archs=["x86_64", "arm64"])
+    FreeBSD = Platform("freebsd", archs=["x86_64", "aarch64"])
 
     LinuxStatic = Platform('linux-static', sdk_name='LINUX_STATIC', archs=[
         'x86_64',
@@ -403,7 +403,7 @@ class StdlibDeploymentTarget(object):
             if machine == 'amd64':
                 return StdlibDeploymentTarget.FreeBSD.x86_64
             elif machine == 'arm64':
-                return StdlibDeploymentTarget.FreeBSD.arm64
+                return StdlibDeploymentTarget.FreeBSD.aarch64
 
         elif system == 'OpenBSD':
             if machine == 'amd64':
