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
    let local_size = get_logical_resolution(window);

    let width = local_size.width;
    let height = (local_size.height as f32 * (2.0 / 5.0)).round() as u32;
    let x = 0;
    let y = local_size.height - height;
    (LogicalPosition::new(x, y), LogicalSize::new(width, height))
}

pub(crate) fn set_window_position_and_size(window: &Window) {
    // window.set_always_on_top(true);

    let local_size = get_logical_resolution(window);

    let width = local_size.width;
    let height = (local_size.height as f32 * (2.0 / 5.0)).round() as u32;
    let x = 0;
    let y = local_size.height - height;

    // println!("size {} {}", local_size.width, local_size.height);
    println!("x {} y {} width{} height{}", x, y, width, height);
    _ = window.set_size(LogicalSize::new(width, height));
    _ = window.set_position(LogicalPosition::new(x, y));
}

pub fn get_active_process_id() -> i32 {
    println!("[{}] in get_active_process_id", Local::now());

    match get_active_window() {
        Ok(active_window) => {
            println!("process_id: {}", active_window.process_id);
            println!("process_name: {}", active_window.process_name);
            println!("title: {}", active_window.title);
            println!("window_id: {}", active_window.window_id);
            let process_id: i32 = active_window.process_id.try_into().unwrap();
            println!("[{}] out get_active_process_id OK", Local::now());
            process_id
        }
        Err(()) => {
            println!("[{}] out get_active_process_id Err", Local::now());
            println!("error occurred while getting the active window");
            0
        }
    }
}

pub fn focus_window(process_id: i32) {
    println!("[{}] in focus_window", Local::now());

    if process_id == 0 {
        println!("process_id is 0");
        println!("[{}] out focus_window", Local::now());
        return;
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let hwnd = winapi::um::winuser::FindWindowW(
            std::ptr::null(),
            process_id.to_string().as_ptr() as _,
        );

        if hwnd != std::ptr::null_mut() {
            winapi::um::winuser::SetForegroundWindow(hwnd);
        }
    }
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::{NSApplicationActivateIgnoringOtherApps, NSRunningApplication};
        use cocoa::base::nil;
        let current_app =
            NSRunningApplication::runningApplicationWithProcessIdentifier(nil, process_id);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        let output = Command::new("wmctrl")
            .arg("-a")
            .arg(format!("pid,{}", process_id))
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
