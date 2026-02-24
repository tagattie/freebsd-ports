--- swift-corelibs-libdispatch/src/swift/Wrapper.swift.orig	2026-01-23 21:08:24 UTC
+++ swift-corelibs-libdispatch/src/swift/Wrapper.swift
@@ -182,7 +182,7 @@ extension DispatchSource : DispatchSourceMachSend,
 }
 #endif
 
-#if !os(Linux) && !os(Android) && !os(Windows)
+#if HAVE_MACH
 extension DispatchSource : DispatchSourceProcess,
 	DispatchSourceFileSystemObject {
 }
@@ -273,7 +273,7 @@ public protocol DispatchSourceMemoryPressure : Dispatc
 }
 #endif
 
-#if !os(Linux) && !os(Android) && !os(Windows)
+#if HAVE_MACH
 public protocol DispatchSourceProcess : DispatchSourceProtocol {
 	var handle: pid_t { get }
 
@@ -303,7 +303,7 @@ public protocol DispatchSourceTimer : DispatchSourcePr
 	func scheduleRepeating(wallDeadline: DispatchWallTime, interval: Double, leeway: DispatchTimeInterval)
 }
 
-#if !os(Linux) && !os(Android) && !os(Windows)
+#if HAVE_MACH
 public protocol DispatchSourceFileSystemObject : DispatchSourceProtocol {
 	var handle: Int32 { get }
 
