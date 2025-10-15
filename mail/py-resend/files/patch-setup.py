The source distribution does not contain requiremtnts.txt.

--- setup.py.orig	2025-10-15 05:35:42 UTC
+++ setup.py
@@ -4,7 +4,7 @@ from resend.version import get_version
 
 from resend.version import get_version
 
-install_requires = open("requirements.txt").readlines()
+#install_requires = open("requirements.txt").readlines()
 
 setup(
     name="resend",
@@ -17,7 +17,7 @@ setup(
     url="https://github.com/resendlabs/resend-python",
     packages=find_packages(exclude=["tests", "tests.*"]),
     package_data={"resend": ["py.typed"]},
-    install_requires=install_requires,
+#    install_requires=install_requires,
     zip_safe=False,
     python_requires=">=3.7",
     keywords=["email", "email platform"],
