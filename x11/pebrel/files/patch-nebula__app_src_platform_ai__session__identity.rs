--- nebula_app/src/platform/ai_session_identity.rs.orig	2026-10-26 13:00:59 UTC
+++ nebula_app/src/platform/ai_session_identity.rs
@@ -130,7 +130,7 @@ pub(crate) fn probe_codex_session(
         return probe_wsl(distro, context.wsl_user(), &pane_id, instance);
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     {
         let _ = shell_pid;
         return probe_local_proc(&pane_id, instance);
@@ -355,6 +355,162 @@ fn probe_local_proc(pane_id: &str, instance: &str) -> 
             let Ok(first_line) = read_first_line(&fd_path) else { continue };
             let fd_number = fd.file_name().to_string_lossy().parse::<u32>().ok();
             let Some(fd_number) = fd_number else { continue };
+            use std::fmt::Write as _;
+            let _ = writeln!(output, "{pid}\t{fd_number}\t{}\t{first_line}", link.display());
+            records += 1;
+            if records >= MAX_PROBE_RECORDS || output.len() >= MAX_PROBE_OUTPUT {
+                return None;
+            }
+        }
+    }
+    parse_probe_context(&output)
+}
+
+#[cfg(target_os = "freebsd")]
+mod freebsd_probe {
+    use std::ffi::CStr;
+    use std::os::unix::ffi::OsStrExt;
+    use std::path::PathBuf;
+
+    pub fn sysctl_bytes(mib: &[libc::c_int]) -> Option<Vec<u8>> {
+        for _ in 0..5 {
+            let mut len: libc::size_t = 0;
+            let rc = unsafe {
+                libc::sysctl(
+                    mib.as_ptr(),
+                    mib.len() as libc::c_uint,
+                    std::ptr::null_mut(),
+                    &mut len,
+                    std::ptr::null(),
+                    0,
+                )
+            };
+            if rc != 0 {
+                return None;
+            }
+            len += len / 8 + 4096;
+            let mut buf = vec![0u8; len];
+            let rc = unsafe {
+                libc::sysctl(
+                    mib.as_ptr(),
+                    mib.len() as libc::c_uint,
+                    buf.as_mut_ptr().cast(),
+                    &mut len,
+                    std::ptr::null(),
+                    0,
+                )
+            };
+            if rc == 0 {
+                buf.truncate(len);
+                return Some(buf);
+            }
+            if std::io::Error::last_os_error().raw_os_error() != Some(libc::ENOMEM) {
+                return None;
+            }
+        }
+        None
+    }
+
+    pub fn list_processes() -> Option<Vec<(u32, String)>> {
+        let buf = sysctl_bytes(&[libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_PROC])?;
+        let mut result = Vec::new();
+        let mut offset = 0;
+        while offset + 4 <= buf.len() {
+            let size = i32::from_ne_bytes(buf[offset..offset + 4].try_into().ok()?) as usize;
+            if size < std::mem::size_of::<libc::kinfo_proc>() || offset + size > buf.len() {
+                break;
+            }
+            let kp: libc::kinfo_proc =
+                unsafe { std::ptr::read_unaligned(buf.as_ptr().add(offset).cast()) };
+            let comm = unsafe { CStr::from_ptr(kp.ki_comm.as_ptr()) }
+                .to_string_lossy()
+                .into_owned();
+            if kp.ki_pid > 0 {
+                result.push((kp.ki_pid as u32, comm));
+            }
+            offset += size;
+        }
+        Some(result)
+    }
+
+    pub fn environ(pid: u32) -> Option<Vec<u8>> {
+        sysctl_bytes(&[libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_ENV, pid as libc::c_int])
+    }
+
+    fn parse_kinfo_file(chunk: &[u8]) -> Option<(libc::c_int, libc::c_int, PathBuf)> {
+        if chunk.len() < std::mem::size_of::<libc::kinfo_file>() {
+            return None;
+        }
+        let kf: libc::kinfo_file = unsafe { std::ptr::read_unaligned(chunk.as_ptr().cast()) };
+        let path = unsafe { CStr::from_ptr(kf.kf_path.as_ptr()) };
+        Some((kf.kf_type, kf.kf_fd, PathBuf::from(std::ffi::OsStr::from_bytes(path.to_bytes()))))
+    }
+
+    pub fn cwd(pid: u32) -> Option<PathBuf> {
+        let buf = sysctl_bytes(&[libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_CWD, pid as libc::c_int])?;
+        let (_, _, path) = parse_kinfo_file(&buf)?;
+        (!path.as_os_str().is_empty()).then_some(path)
+    }
+
+    pub fn open_files(pid: u32) -> Option<Vec<(u32, PathBuf)>> {
+        let buf =
+            sysctl_bytes(&[libc::CTL_KERN, libc::KERN_PROC, libc::KERN_PROC_FILEDESC, pid as libc::c_int])?;
+        let mut result = Vec::new();
+        let mut offset = 0;
+        while offset + 4 <= buf.len() {
+            let size = i32::from_ne_bytes(buf[offset..offset + 4].try_into().ok()?) as usize;
+            if size == 0 || offset + size > buf.len() {
+                break;
+            }
+            if let Some((kind, fd, path)) = parse_kinfo_file(&buf[offset..offset + size]) {
+                if kind == libc::KF_TYPE_VNODE && fd >= 0 && !path.as_os_str().is_empty() {
+                    result.push((fd as u32, path));
+                }
+            }
+            offset += size;
+        }
+        Some(result)
+    }
+}
+
+#[cfg(target_os = "freebsd")]
+fn probe_local_proc(pane_id: &str, instance: &str) -> Option<CodexSession> {
+    let deadline = Instant::now() + PROBE_TIMEOUT;
+    let mut output = String::new();
+    let mut records = 0;
+    for (pid, comm) in freebsd_probe::list_processes()? {
+        if Instant::now() >= deadline {
+            return None;
+        }
+        if !is_codex_process_name(comm.trim()) {
+            continue;
+        }
+        let Some(environment) = freebsd_probe::environ(pid) else { continue };
+        if !environment_has_value(&environment, b"PEBREL_PANE_ID", pane_id.as_bytes())
+            && !environment_has_value(&environment, b"NEBULA_PANE_ID", pane_id.as_bytes())
+        {
+            continue;
+        }
+        if !environment_has_value(
+            &environment,
+            crate::agent_env::PROCESS_ENV.as_bytes(),
+            instance.as_bytes(),
+        ) {
+            continue;
+        }
+        let Some(codex_home) = codex_home_from_environment(&environment) else {
+            continue;
+        };
+        let Some(cwd) = freebsd_probe::cwd(pid) else { continue };
+        let Ok(sessions_root) = cwd.join(codex_home).join("sessions").canonicalize() else {
+            continue;
+        };
+        let Some(files) = freebsd_probe::open_files(pid) else { continue };
+        for (fd_number, link) in files {
+            if !is_rollout_link(&link, &sessions_root) {
+                continue;
+            }
+            let Ok(first_line) = read_first_line(&link) else { continue };
             use std::fmt::Write as _;
             let _ = writeln!(output, "{pid}\t{fd_number}\t{}\t{first_line}", link.display());
             records += 1;
