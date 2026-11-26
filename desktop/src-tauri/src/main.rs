// Hide the extra console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri_runtime_cef::cef_entry_point]
fn main() {
    nexum_desktop_lib::run()
}
