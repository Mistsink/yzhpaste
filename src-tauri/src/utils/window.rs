use tauri::LogicalPosition;
use tauri::LogicalSize;
use tauri::PhysicalSize;
use tauri::PhysicalPosition;
use tauri::Window;

fn get_logical_resolution(window: &Window) -> LogicalSize<u32>{
    let monitor = window.current_monitor().unwrap().expect("No monitor found");
    let inner_size = monitor.size();
    let scale_factor = monitor.scale_factor();

    println!("inner_size: {:?}", inner_size);
    println!("scale_factor: {:?}", scale_factor);
    println!("monitor_size: {:?}", monitor.size());

    let logical_width = (inner_size.width as f64 / scale_factor) as u32;
    let logical_height = (inner_size.height as f64 / scale_factor) as u32;

    LogicalSize::new(logical_width, logical_height)
}

pub(crate) fn set_window_position_and_size(window: &Window) {
    // window.set_always_on_top(true);

    let local_size = get_logical_resolution(window);

    let width = local_size.width;
    let height = (local_size.height as f32 * (1.0 / 3.0)).round() as u32;
    let x = 0;
    let y = local_size.height - height;


    println!("size {} {}", local_size.width, local_size.height);
    println!("x {} y {} width{} height{}", x, y, width, height);
    _ = window.set_size(LogicalSize::new(width, height));
    _ = window.set_position(LogicalPosition::new(x, y));
}
