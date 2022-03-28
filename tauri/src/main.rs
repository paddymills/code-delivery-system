#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn load_job_shipment() -> Result<String, String> {
  Ok("job-shipment loaded".into())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load_job_shipment])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
