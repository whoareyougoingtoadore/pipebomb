use std::process::Command;

#[cfg(unix)]
fn ensure_this_is_root_to_nuke_haha() {
    if unsafe { libc::geteuid() } == 0 {
        return;
    }

    let exe = std::env::current_exe().expect("WHO'S HE??????? WHERE ARE YOU BRO WTF");

    #[cfg(target_os = "linux")]
    let status = Command::new("pkexec")
        .arg(exe)
        .status();

    #[cfg(target_os = "macos")]
    let status = {
        let exe_str = exe
            .display()
            .to_string()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let script = format!(
            "do shell script (quoted form of \"{exe_str}\") & \" > /dev/null 2>&1 &\" with administrator privileges"
        );
        Command::new("osascript").args(["-e", &script]).status()
    };

    match status {
        Ok(s) if s.success() => std::process::exit(0),
        _ => std::process::exit(1),
    }
}

#[tauri::command]
fn am_i_admin() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }

    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
        use windows_sys::Win32::Security::{
            GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY
        };
        use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

        unsafe {
            let mut token: HANDLE = std::ptr::null_mut();

            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
                return false;
            }

            let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
            let ok = GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut 0u32,
            );
            CloseHandle(token);
            ok != 0 && elevation.TokenIsElevated != 0
        }
    }
}

#[tauri::command]
fn is_ts_a_vm() -> bool {
    const YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL: [&str; 6] = ["vmware", "virtualbox", "parallels", "qemu", "innotek", "virtual machine"]; // hope they are correct </3

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = Command::new("sysctl")
            .args(["-n", "kern.hv_vmm_present"])
            .output()
        {
            if String::from_utf8_lossy(&out.stdout).trim() == "1" {
                eprintln!("[vm-detect] flagged by: kern.hv_vmm_present == 1");
                return true;
            }
        }

        if let Ok(out) = Command::new("ioreg").args(["-l"]).output() {
            let res = String::from_utf8_lossy(&out.stdout).to_lowercase();

            if let Some(furry) = YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL.iter().find(|f| res.contains(**f)) {
                eprintln!("[vm-detect] flagged by: ioreg contains \"{furry}\"");
                return true;
            }
        }

        false
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(out) = Command::new("systemd-detect-virt").output() {
            let parsethatpls = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            if !parsethatpls.is_empty() && parsethatpls != "none" {
                return true;
            }
        }

        const sysinfo: [&str; 3] = [
            "/sys/class/dmi/id/product_name",
            "/sys/class/dmi/id/sys_vendor",
            "/sys/class/dmi/id/board_vendor",
        ];

        for path in sysinfo {
            if let Ok(content) = std::fs::read_to_string(path) {
                let content = content.to_lowercase();
                if YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL.iter().any(|furry| content.contains(furry)) {
                    return true;
                }
            }
        }

        false
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(out) = Command::new("wmic")
            .args(["computersystem", "get", "manufacturer,model"])
            .output()
        {
            let furry = String::from_utf8_lossy(&out.stdout).to_lowercase();
            if YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL.iter().any(|nyah| furry.contains(nyah)) {
                return true;
            }
        }

        false
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows"
    )))]
    {
        false // WHO IS HE??
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(all(unix, not(debug_assertions)))]
    ensure_this_is_root_to_nuke_haha();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_ts_a_vm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
