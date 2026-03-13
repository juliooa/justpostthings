#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::get_settings,
            commands::save_settings,
            commands::translate_preview,
            commands::upload_image,
            commands::submit_post,
            commands::shrink_text,
            commands::read_image_base64,
            commands::list_ideas,
            commands::create_idea,
            commands::update_idea,
            commands::delete_idea,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
