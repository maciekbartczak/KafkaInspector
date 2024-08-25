#[derive(serde::Deserialize)]
struct ConnectToClusterParams {
    address: String,
}

#[tauri::command]
fn connect(params: ConnectToClusterParams) -> bool {
    println!("connect to cluster");
    println!("{}", params.address);
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
