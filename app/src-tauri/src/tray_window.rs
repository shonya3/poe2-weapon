use tauri::{AppHandle, PhysicalPosition, WebviewWindowBuilder};

pub fn create_tray_window(handle: &AppHandle) {
    let w_width = 300.;
    let w_height = 200.;

    let window = WebviewWindowBuilder::new(
        handle,
        "TrayWindow",
        tauri::WebviewUrl::App("/tray-window".into()),
    )
    .title("PoE2 Weapon")
    .visible(false)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .always_on_top(true)
    .inner_size(w_width, w_height)
    .build()
    .unwrap();

    if let Ok(Some(monitor)) = handle.primary_monitor() {
        let m_size = monitor.size();
        let position = PhysicalPosition {
            x: m_size.width as f64 - w_width - 100.,
            y: m_size.height as f64 - w_height - 50.,
        };
        window.set_position(position).ok();
        window.show().ok();
    }
}
