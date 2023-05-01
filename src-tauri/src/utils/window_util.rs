use std::{thread, time};

use active_win_pos_rs::get_active_window;
use bezier_rs::Bezier;
use cocoa::appkit::{NSApplicationActivateIgnoringOtherApps, NSRunningApplication};
use cocoa::base::nil;
use tauri::LogicalSize;
use tauri::Window;
use tauri::{AppHandle, LogicalPosition, WindowBuilder};
fn slide_up(window: &Window) {
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let monitor_size = monitor.size();

    let duration = time::Duration::from_millis(20);
    let start_y = monitor_size.height as f64;
    let end_y = (monitor_size.height - 300) as f64; // 假设窗口高度为300

    let bezier = Bezier::from_cubic_coordinates(0.0, start_y, 0.1, start_y, 0.9, end_y, 1.0, end_y);

    for t in (0..=100).map(|i| i as f64 / 100.0) {
        let position = bezier.evaluate(bezier_rs::TValue::Parametric(t));
        _ = window.set_position(LogicalPosition::new(0, position.y as i32));
        thread::sleep(duration);
    }
}

fn slide_down(window: &Window) {
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
    // println!("x {} y {} width{} height{}", x, y, width, height);
    _ = window.set_size(LogicalSize::new(width, height));
    _ = window.set_position(LogicalPosition::new(x, y));
}

pub fn get_active_process_id() -> i32 {
    match get_active_window() {
        Ok(active_window) => {
            let process_id: i32 = active_window.process_id.try_into().unwrap();
            process_id
        }
        Err(()) => {
            println!("error occurred while getting the active window");
            0
        }
    }
}

pub fn focus_window(process_id: i32) {
    if process_id == 0 {
        return;
    }
    #[cfg(target_os = "macos")]
    unsafe {
        let current_app =
            NSRunningApplication::runningApplicationWithProcessIdentifier(nil, process_id);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
    }
}
