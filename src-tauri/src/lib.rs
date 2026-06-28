use std::process::Command;

#[tauri::command]
fn is_ts_a_vm() -> bool {
    const YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL: [&str; 9] = ["vmware", "virtualbox", "parallels", "qemu", "innotek", "utm", "virtual machine", "kvm", "xen"]; // hope they are correct </3

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = Command::new("sysctl")
            .args(["-n", "kern.hv_vmm_present"])
            .output()
        {
            if String::from_utf8_lossy(&out.stdout).trim() == "1" {
                return true;
            }
        }

        if let Ok(out) = Command::new("ioreg").args(["-l"]).output() {
            let res = String::from_utf8_lossy(&out.stdout).to_lowercase();

            if YOUKNOWSOMETIMESIWONDERHOWITWOULDFEELTOBEAGIRLNGL.iter().any(|furry| res.contains(furry)) {
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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_ts_a_vm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
