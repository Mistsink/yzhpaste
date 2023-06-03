use chrono::Local;

#[cfg(target_os = "windows")]
mod windows {
    use std::os::windows::io::AsRawHandle;
    use sysinfo::{Pid, ProcessExt, System, SystemExt};
    use winapi::shared::minwindef::FALSE;
    use winapi::shared::windef::HWND;
    use winapi::um::dwmapi::DwmGetWindowAttribute;
    use winapi::um::winuser::{FindWindowExW, GetWindowTextW};

    const DWMWA_CLOAKED: u32 = 14;

    pub fn is_window_visible(process_id: u32) -> Option<bool> {
        let sys = System::new_all();
        let process = sys.processes().get(&Pid::from(process_id as usize))?;

        // 获取窗口句柄
        let hwnd = unsafe {
            FindWindowExW(
                HWND::NULL,
                HWND::NULL,
                std::ptr::null_mut(),
                process.name().as_ptr(),
            )
        };

        if !hwnd.is_null() {
            let mut is_cloaked: u32 = 0;

            let result = unsafe {
                DwmGetWindowAttribute(
                    hwnd,
                    DWMWA_CLOAKED,
                    &mut is_cloaked as *mut u32 as *mut _,
                    std::mem::size_of::<u32>() as u32,
                )
            };

            if result == 0 && is_cloaked == FALSE {
                return Some(true);
            }
        }

        Some(false)
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::process::Command;
    use sysinfo::{Pid, ProcessExt, System, SystemExt};

    #[allow(unused)]
    pub fn is_window_visible(process_id: u32) -> Option<bool> {
        let sys = System::new_all();
        let process = sys.processes().get(&Pid::from(process_id as usize))?;
        println!("process name: {}", process.name());

        // 获取窗口列表
        let output = Command::new("osascript")
            .arg("-e")
            .arg(&format!(
                "tell application \"System Events\" to get the visible of every window of every process whose name is \"{}\"",
                process.name()
            ))
            .output()
            .expect("Failed to execute command");

        let windows: Vec<_> = String::from_utf8_lossy(&output.stdout)
            .split(",")
            .map(|s| s.trim().parse::<bool>().unwrap_or(false))
            .collect();

        for window in windows {
            if window {
                return Some(true);
            }
        }

        Some(false)
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::process::Command;
    use sysinfo::{Pid, ProcessExt, System, SystemExt};

    pub fn is_window_visible(process_id: u32) -> Option<bool> {
        let sys = System::new_all();
        let process = sys.processes().get(&Pid::from(process_id as usize))?;

        // 获取窗口 ID
        let output = Command::new("xwininfo")
            .arg("-name")
            .arg(process.name())
            .arg("-root")
            .output()
            .expect("Failed to execute command");

        let id = String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.starts_with("xwininfo: Window id:"))
            .and_then(|line| line.split(":").nth(1))
            .and_then(|id| id.trim().parse::<u64>().ok())?;

        // 获取窗口可见性
        let output = Command::new("xprop")
            .arg("-id")
            .arg(&format!("{}", id))
            .arg("_NET_WM_STATE")
            .output()
            .expect("Failed to execute command");

        let visible = String::from_utf8_lossy(&output.stdout).contains("_NET_WM_STATE_VISIBLE");

        Some(visible)
    }
}

#[allow(unused)]
pub fn is_window_visible(process_id: u32) -> Option<bool> {
    println!("[{}] in is_window_visible", Local::now());
    #[cfg(target_os = "windows")]
    let result = windows::is_window_visible(process_id);

    #[cfg(target_os = "macos")]
    let result = macos::is_window_visible(process_id);

    #[cfg(target_os = "linux")]
    let result = linux::is_window_visible(process_id);

    println!("[{}] out is_window_visible", Local::now());

    result
}
