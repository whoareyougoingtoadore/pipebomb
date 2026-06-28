#[tauri::command]
fn is_ts_a_vm() -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        if let Ok(out) = Command::new("ioreg").args(["-l"]).output() {
            let res = String::from_utf8_lossy(&out.stdout)
            const turipipip: [&str; 5] = ["vmware", "virtualbox", "parallels", "qemu", "innotek", "utm"] // hope they are correct </3

            if turipipip.iter().any(|furry| res.contains(furry)) {
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
