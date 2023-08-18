use active_win_pos_rs::get_active_window;
use bezier_rs::Bezier;
use chrono::Local;
use std::{thread, time};
use tauri::LogicalPosition;
use tauri::Window;
use tauri::{LogicalSize, PhysicalPosition};

#[allow(unused)]
pub fn slide_up(window: &Window) {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let monitor_size = monitor.size();

    let win_y = window.outer_position().unwrap().y;
    println!("inner_position {:?}", window.inner_position());
    println!("outter_position {:?}", window.outer_position());
    println!("inner_size {:?}", window.inner_size());
    println!("win_y: {}", win_y);

    let duration = time::Duration::from_millis(20);
    // let start_y = monitor_size.height as f64;
    // let end_y = (monitor_size.height - 300) as f64; // 假设窗口高度为300
    let start_y = win_y as f64;
    let end_y = 0.0; // 假设窗口高度为300

    let bezier = Bezier::from_cubic_coordinates(0.0, start_y, 0.1, start_y, 0.9, end_y, 1.0, end_y);

    for t in (0..=100).map(|i| i as f64 / 100.0) {
        let position = bezier.evaluate(bezier_rs::TValue::Parametric(t));
        _ = window.set_position(PhysicalPosition::new(0, position.y as i32));
        thread::sleep(duration);
    }
}

#[allow(unused)]
pub fn slide_down(window: &Window) {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let monitor_size = monitor.size();

    let duration = time::Duration::from_millis(20);
    let start_y = (monitor_size.height - 300) as f64; // 假设窗口高度为300
    let end_y = monitor_size.height as f64;

    let bezier = Bezier::from_cubic_coordinates(0.0, start_y, 0.1, start_y, 0.9, end_y, 1.0, end_y);

    for t in (0..=100).map(|i| i as f64 / 100.0) {
        let position = bezier.evaluate(bezier_rs::TValue::Parametric(t));
        _ = window.set_position(LogicalPosition::new(0, position.y as i32));
        thread::sleep(duration);
    }
}

fn logical2physical(window: &Window, size: u32) -> u32 {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let scale_factor = monitor.scale_factor();
    (size as f64 * scale_factor) as u32
}

fn physical2logical(window: &Window, size: u32) -> u32 {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let scale_factor = monitor.scale_factor();
    (size as f64 / scale_factor) as u32
}

fn get_logical_resolution(window: &Window) -> LogicalSize<u32> {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");

    let inner_size = monitor.size();
    let scale_factor = monitor.scale_factor();

    // println!("inner_size: {:?}", inner_size);
    // println!("scale_factor: {:?}", scale_factor);
    // println!("monitor_size: {:?}", monitor.size());

    let logical_width = (inner_size.width as f64 / scale_factor) as u32;
    let logical_height = (inner_size.height as f64 / scale_factor) as u32;

    LogicalSize::new(logical_width, logical_height)
}

#[allow(unused)]
pub fn get_window_position_and_size(window: &Window) -> (LogicalPosition<u32>, LogicalSize<u32>) {
    let logical_size = get_logical_resolution(window);

    let width = logical_size.width;
    let height = (logical_size.height as f32 * (2.0 / 5.0)).round() as u32;
    let x = 0;
    let y = logical_size.height - height;
    (LogicalPosition::new(x, y), LogicalSize::new(width, height))
}

#[cfg(target_os = "windows")]
use winapi::shared::windef::RECT;
use winapi::um::winuser::{SystemParametersInfoW, SPI_GETWORKAREA};
fn get_workarea_dimensions() -> (i32, i32, i32, i32) {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe {
        SystemParametersInfoW(SPI_GETWORKAREA, 0, &mut rect as *mut _ as *mut _, 0);
    }

    (rect.left, rect.top, rect.right, rect.bottom)
}

pub(crate) fn set_window_position_and_size(window: &Window) {
    // window.set_always_on_top(true);

    let logical_size = get_logical_resolution(window);

    let width = logical_size.width;
    let mut height = (logical_size.height as f32 * (2.0 / 5.0)).round() as u32;
    let x = 0;
    let y = logical_size.height - height;

    #[cfg(target_os = "windows")]
    {
        let (left, top, right, bottom) = get_workarea_dimensions();
        println!(
            "left: {}, top: {}, right: {}, bottom: {}",
            left, top, right, bottom
        );
        println!(
            "logical: left: {}, top: {}, right: {}, bottom: {}",
            physical2logical(window, left as u32),
            physical2logical(window, top as u32),
            physical2logical(window, right as u32),
            physical2logical(window, bottom as u32)
        );
        println!("x: {}, y: {}, width: {}, height: {}, logical_size.height: {}", x, y, width, height, logical_size.height);
        height = physical2logical(window, bottom as u32) - y;
        println!("height: {}", height);
        height -= 20;
        println!("height: {}", height);

        // windows::get_taskbar_info().map(|(taskbar_height, visible)| {
        //     if visible {
        //         println!("taskbar_height: {}", taskbar_height);
        //         height = height - physical2logical(window, taskbar_height);
        //         println!("height: {}", height);
        //     }
        // });
    }

    // println!("size {} {}", logical_size.width, logical_size.height);
    println!("x {} y {} width{} height{}", x, y, width, height);
    _ = window.set_position(LogicalPosition::new(x, y));
    _ = window.set_size(LogicalSize::new(width, height));
}

#[cfg(target_os = "windows")]
mod windows {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::shared::windef::{HWND, RECT};
    use winapi::um::winuser::{
        FindWindowW, GetClassNameW, GetWindowRect, GetWindowTextW, IsWindowVisible,
    };

    pub fn get_taskbar_info() -> Option<(u32, bool)> {
        let mut taskbar: HWND;
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };

        unsafe {
            let shell_tray_wnd: Vec<u16> = OsStr::new("Shell_TrayWnd")
                .encode_wide()
                .chain(Some(0))
                .collect();
            taskbar = FindWindowW(shell_tray_wnd.as_ptr(), std::ptr::null());
            if taskbar.is_null() {
                return None;
            }

            GetWindowRect(taskbar, &mut rect);
        }

        // physical size
        let height = (rect.bottom - rect.top) as u32;
        let visible = unsafe { IsWindowVisible(taskbar) } != 0;

        Some((height, visible))
    }

    pub fn get_window_text(hwnd: HWND) -> Result<String, &'static str> {
        let mut buffer = [0u16; 1024]; // 缓冲区用于存储窗口文本
        let len = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
        if len == 0 {
            return Err("Failed to get window text");
        }
        let text = String::from_utf16(&buffer[..len as usize])
            .map_err(|_| "Failed to convert window text to UTF-16")?;
        Ok(text)
    }

    pub fn get_class_name(hwnd: HWND) -> Result<String, &'static str> {
        let mut buffer = [0u16; 256]; // 缓冲区用于存储窗口类名
        let len = unsafe { GetClassNameW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
        if len == 0 {
            return Err("Failed to get window class name");
        }
        let class_name = String::from_utf16(&buffer[..len as usize])
            .map_err(|_| "Failed to convert window class name to UTF-16")?;
        Ok(class_name)
    }

    pub fn get_hwnd_from_class_and_title(
        class_name: &str,
        window_name: &str,
    ) -> Result<HWND, &'static str> {
        let class_name: Vec<u16> = OsStr::new(class_name)
            .encode_wide()
            .chain(Some(0))
            .collect();
        let window_name: Vec<u16> = OsStr::new(window_name)
            .encode_wide()
            .chain(Some(0))
            .collect();

        let hwnd = unsafe { FindWindowW(class_name.as_ptr(), window_name.as_ptr()) };

        if hwnd.is_null() {
            Err("Could not find window")
        } else {
            Ok(hwnd)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProcessInfo {
    pub process_id: i32,
    pub window_id: String,
    pub class_name_win: String,
    pub window_text_win: String,
}

pub fn get_active_process_info() -> ProcessInfo {
    println!("[{}] in get_active_process_id", Local::now());

    match get_active_window() {
        Ok(active_window) => {
            let mut class_name = "".to_string();
            let mut window_text = "".to_string();

            #[cfg(target_os = "windows")]
            unsafe {
                use winapi::um::winuser::GetForegroundWindow;
                let hwnd = GetForegroundWindow();
                class_name = match windows::get_class_name(hwnd) {
                    Ok(class_name) => class_name,
                    Err(_) => "".to_string(),
                };
                println!("class_name: {}", class_name);
                window_text = match windows::get_window_text(hwnd) {
                    Ok(text) => text,
                    Err(_) => {
                        // 获取窗口文本失败时，将 window_text 和 class_name 都设置为空字符串
                        class_name = "".to_string();
                        "".to_string()
                    }
                };
            }

            // active_window
            let process_id: i32 = if class_name.is_empty() && window_text.is_empty() {
                0
            } else {
                active_window.process_id.try_into().unwrap()
            };

            println!("[{}] out get_active_process_id OK", Local::now());
            ProcessInfo {
                process_id,
                window_id: active_window.window_id,
                class_name_win: class_name,
                window_text_win: window_text,
            }
        }
        Err(()) => {
            println!("[{}] out get_active_process_id Err", Local::now());
            println!("error occurred while getting the active window");
            ProcessInfo::default()
        }
    }
}

pub fn focus_window(process_info: &ProcessInfo) {
    println!("[{}] in focus_window", Local::now());

    if process_info.process_id == 0 {
        println!("process_id is 0");
        println!("[{}] out focus_window", Local::now());
        return;
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let hwnd_result = windows::get_hwnd_from_class_and_title(
            &process_info.class_name_win,
            &process_info.window_text_win,
        );

        println!(
            "name: {}\nwindow_text_win{}",
            process_info.class_name_win, process_info.window_text_win
        );

        match hwnd_result {
            Ok(hwnd) => {
                if hwnd != std::ptr::null_mut() {
                    winapi::um::winuser::SetForegroundWindow(hwnd);
                }
            }
            Err(e) => {
                // 在这里处理错误，你可以打印出错误信息，或者采取其他适合你应用程序的错误处理策略
                eprintln!("Failed to get HWND: {:?}", e);
            }
        }
    }
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::{NSApplicationActivateIgnoringOtherApps, NSRunningApplication};
        use cocoa::base::nil;
        let current_app = NSRunningApplication::runningApplicationWithProcessIdentifier(
            nil,
            process_info.process_id,
        );
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("wmctrl")
            .arg("-a")
            .arg(format!("pid,{}", process_info.process_id))
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Focused window successfully.");
        } else {
            println!("Failed to focus window.");
        }
    }
    println!("[{}] out focus_window", Local::now());
}
