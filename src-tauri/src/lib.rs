#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Show the main window
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit", true, Option::<&str>::None)?;
            let show = MenuItem::with_id(app, "show", "Show", true, Option::<&str>::None)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::DoubleClick { .. } = event {
                        let app = tray.app_handle();
                        let window = app.get_webview_window("main").unwrap();
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        // // Register the desktop tray
        // .system_tray(desktop_tray)
        // // Add events for the desktop tray
        // .on_system_tray_event(|app, event| match event {
        //     // If the event is a left click into the icon,
        //     // verify that the window is closed and re-open it
        //     // while setting it up again (Tauri doesn't save
        //     // the status of hidden windows).
        //     SystemTrayEvent::LeftClick {
        //         position: _,
        //         size: _,
        //         ..
        //     } => {
        //         let window = app.get_window("main").unwrap();
        //         // If the window is closed
        //         if !window.is_visible().unwrap() {
        //             // Re-open the window
        //             window.show().unwrap();
        //         }
        //         // Even if the window isn't closed, it could be minimized in the taskbar; set it as focused
        //         else {
        //             window.set_focus().unwrap();
        //         }
        //     }
        //     // If the event is a click to an item
        //     SystemTrayEvent::MenuItemClick { id, .. } => {
        //         // We compare the name of the item to...
        //         match id.as_str() {
        //             // `quit` is the item with the string `Quit Discord`
        //             // If the user clicks quit, close discord-tauri
        //             "quit" => {
        //                 std::process::exit(0);
        //             }
        //             _ => {}
        //         }
        //     }
        //     _ => {}
        // })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
