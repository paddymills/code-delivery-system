#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use cds::{Project, test_data, JobData};

use serde::{Serialize, Deserialize};
use std::fmt::Write;

#[derive(Serialize, Deserialize)]
struct CdsResult<T> {
  msg: String,
  payload: T
}

#[tauri::command]
fn get_projects() -> Result<CdsResult<Vec<Project>>, String> {
  let result = CdsResult::<Vec<Project>> { msg: "Success".into(), payload: test_data() };

  Ok(result)
}

#[tauri::command]
fn load_job_shipment(project: Project) -> Result<CdsResult<JobData>, String> {
  // build status
  let mut res = String::from("Loaded: ");
  res.push_str(&project.name);
  res.push('-');
  match write!(res, "{}", project.shipments[0]) {
    Ok(_) => Ok(
      CdsResult {
        msg: res,
        payload: JobData::test_data(project.name)
      }
    ),
    Err(err) => Err(err.to_string())
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_projects,
      load_job_shipment
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
