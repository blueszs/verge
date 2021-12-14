use tauri::api::process::kill_children;

use crate::utils::{clash, import};

#[tauri::command]
pub fn cmd_restart_sidebar() {
  kill_children();
  clash::run_clash_bin();
}

#[tauri::command]
pub async fn cmd_import_profile(url: String) -> Result<String, String> {
  match import::import_profile(&url).await {
    Ok(_) => Ok(String::from("success")),
    Err(_) => Err(String::from("error")),
  }
}
