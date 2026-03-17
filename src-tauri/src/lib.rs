#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSScreen};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

use notify::{Watcher, RecursiveMode, Event};
use tauri::{Manager, menu::{Menu, MenuItem}, tray::TrayIconBuilder, WebviewUrl, WebviewWindowBuilder, State};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_store::StoreExt;
use serde_json::Value;

// --- RUST COMMANDS ---

#[tauri::command]
async fn get_tasks(app: tauri::AppHandle) -> Result<Value, String> {
    let store = app.store("tasks.json").map_err(|e| e.to_string())?;
    let data = store.get("tasksData").unwrap_or(Value::Null);
    Ok(data)
}

#[tauri::command]
async fn save_tasks(app: tauri::AppHandle, data: Value) -> Result<(), String> {
    let store = app.store("tasks.json").map_err(|e| e.to_string())?;
    store.set("tasksData", data);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .invoke_handler(tauri::generate_handler![get_tasks, save_tasks])
    .setup(|app| {
      let main_window = match app.get_webview_window("main") {
        Some(w) => w,
        None => return Ok(()),
      };

      #[cfg(target_os = "macos")]
      {
        if let Ok(ns_window) = main_window.ns_window() {
          let ns_id = ns_window as id;
          unsafe {
            let _: () = msg_send![ns_id, setStyleMask: 0]; 
            let _: () = msg_send![ns_id, setLevel: -2147483624]; 
            
            let screen = NSScreen::mainScreen(nil);
            let frame = NSScreen::frame(screen);
            let _: () = msg_send![ns_id, setFrame: frame display: true];

            let behavior = NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
              | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
              | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle
              | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary;
            ns_id.setCollectionBehavior_(behavior);
            
            let _: () = msg_send![ns_id, setIgnoresMouseEvents: true];
            let _: () = msg_send![ns_id, setHasShadow: false];
            let _: () = msg_send![ns_id, setBackgroundColor: cocoa::appkit::NSColor::clearColor(nil)];
            let _: () = msg_send![ns_id, setOpaque: false];
            let _: () = msg_send![ns_id, setCanHide: false];
          }
        }
      }

      // --- TRAY ICON ---
      let edit_i = MenuItem::with_id(app, "edit", "Edit Tasks", true, None::<&str>)?;
      let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&edit_i, &quit_i])?;

      let mut tray_builder = TrayIconBuilder::new().menu(&menu).show_menu_on_left_click(true);
      if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
      }

      let _tray = tray_builder.on_menu_event(move |app, event| {
          match event.id.as_ref() {
            "edit" => {
              if let Some(editor) = app.get_webview_window("editor") {
                let _ = editor.set_focus();
              } else {
                let _ = WebviewWindowBuilder::new(app, "editor", WebviewUrl::App("editor.html".into()))
                  .title("Task Editor")
                  .inner_size(600.0, 800.0)
                  .resizable(true)
                  .build();
              }
            }
            "quit" => {
              app.exit(0);
            }
            _ => {}
          }
        })
        .build(app)?;

      // --- FILE WATCHER ---
      let window_for_watcher = main_window.clone();
      let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            let should_reload = event.paths.iter().any(|p| {
              let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
              ext == "js" || ext == "html"
            });
            if should_reload && (event.kind.is_modify() || event.kind.is_create()) {
              let _ = window_for_watcher.eval("window.location.reload()");
            }
        }
      })?;

      let mut watch_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
      if watch_path.ends_with("src-tauri") { watch_path.pop(); }
      watch_path.push("src");

      if watch_path.exists() {
        let _ = watcher.watch(&watch_path, RecursiveMode::Recursive);
      }
      app.manage(watcher);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
