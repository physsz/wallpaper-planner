#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSScreen};
#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};

use notify::{Watcher, RecursiveMode, Event};
use std::sync::Mutex;
use tauri::{Emitter, Manager, menu::{Menu, MenuItem}, tray::TrayIconBuilder, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;
use serde_json::Value;

#[cfg(target_os = "macos")]
const DESKTOP_WINDOW_LEVEL: i32 = -2147483624;
#[cfg(target_os = "macos")]
const EDIT_WINDOW_LEVEL: i32 = 4;
const DASHBOARD_EDIT_MENU_ID: &str = "toggle_dashboard_edit";
const DASHBOARD_EDIT_MENU_LABEL: &str = "Enable Dashboard Edit Mode";
const VIEW_ROLLING_MENU_ID: &str = "set_view_rolling";
const VIEW_FIXED_MENU_ID: &str = "set_view_fixed";
const VIEW_ROLLING_LABEL: &str = "Rolling View";
const VIEW_FIXED_LABEL: &str = "Fixed View";

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

fn get_saved_view_mode(data: &Value) -> &str {
    data.get("appearance")
        .and_then(|appearance| appearance.get("viewMode"))
        .and_then(Value::as_str)
        .unwrap_or("rolling")
}

fn set_saved_view_mode(data: &mut Value, mode: &str) {
    if !data.is_object() {
        *data = serde_json::json!({});
    }

    let object = data.as_object_mut().expect("tasksData must be an object");
    let appearance = object
        .entry("appearance".to_string())
        .or_insert_with(|| serde_json::json!({}));

    if !appearance.is_object() {
        *appearance = serde_json::json!({});
    }

    if let Some(appearance_object) = appearance.as_object_mut() {
        appearance_object.insert("viewMode".to_string(), Value::String(mode.to_string()));
    }
}

#[tauri::command]
async fn get_dashboard_edit_mode(state: tauri::State<'_, Mutex<bool>>) -> Result<bool, String> {
    let current = state.lock().map_err(|e| e.to_string())?;
    Ok(*current)
}

#[tauri::command]
async fn set_dashboard_edit_mode(
    app: tauri::AppHandle,
    state: tauri::State<'_, Mutex<bool>>,
    enabled: bool,
) -> Result<bool, String> {
    {
        let mut current = state.lock().map_err(|e| e.to_string())?;
        *current = enabled;
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(window) = app.get_webview_window("main") {
            if let Ok(ns_window) = window.ns_window() {
                let ns_id = ns_window as id;
                unsafe {
                    let level = if enabled { EDIT_WINDOW_LEVEL } else { DESKTOP_WINDOW_LEVEL };
                    let _: () = msg_send![ns_id, setLevel: level];
                    let _: () = msg_send![ns_id, setIgnoresMouseEvents: !enabled];
                }
            }
        }
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.emit("dashboard-edit-mode-changed", enabled);
    }

    Ok(enabled)
}

fn update_dashboard_edit_menu<R: tauri::Runtime>(
    item: &tauri::menu::MenuItem<R>,
    enabled: bool,
) -> tauri::Result<()> {
    let label = if enabled {
        format!("✓ {DASHBOARD_EDIT_MENU_LABEL}")
    } else {
        DASHBOARD_EDIT_MENU_LABEL.to_string()
    };
    item.set_text(label)
}

fn update_view_mode_menus<R: tauri::Runtime>(
    rolling_item: &tauri::menu::MenuItem<R>,
    fixed_item: &tauri::menu::MenuItem<R>,
    mode: &str,
) -> tauri::Result<()> {
    let rolling_label = if mode == "rolling" {
        format!("✓ {VIEW_ROLLING_LABEL}")
    } else {
        VIEW_ROLLING_LABEL.to_string()
    };
    let fixed_label = if mode == "fixed" {
        format!("✓ {VIEW_FIXED_LABEL}")
    } else {
        VIEW_FIXED_LABEL.to_string()
    };

    rolling_item.set_text(rolling_label)?;
    fixed_item.set_text(fixed_label)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .manage(Mutex::new(false))
    .invoke_handler(tauri::generate_handler![
      get_tasks,
      save_tasks,
      get_dashboard_edit_mode,
      set_dashboard_edit_mode
    ])
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
            let _: () = msg_send![ns_id, setLevel: DESKTOP_WINDOW_LEVEL]; 
            
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
      let toggle_dashboard_i = MenuItem::with_id(app, DASHBOARD_EDIT_MENU_ID, DASHBOARD_EDIT_MENU_LABEL, true, None::<&str>)?;
      let rolling_view_i = MenuItem::with_id(app, VIEW_ROLLING_MENU_ID, VIEW_ROLLING_LABEL, true, None::<&str>)?;
      let fixed_view_i = MenuItem::with_id(app, VIEW_FIXED_MENU_ID, VIEW_FIXED_LABEL, true, None::<&str>)?;
      let edit_i = MenuItem::with_id(app, "edit", "Edit Tasks", true, None::<&str>)?;
      let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
      let menu = Menu::with_items(app, &[&toggle_dashboard_i, &rolling_view_i, &fixed_view_i, &edit_i, &quit_i])?;

      let current_view_mode = {
        let store = app.store("tasks.json")?;
        let data = store.get("tasksData").unwrap_or(Value::Null);
        get_saved_view_mode(&data).to_string()
      };
      let _ = update_view_mode_menus(&rolling_view_i, &fixed_view_i, &current_view_mode);

      let mut tray_builder = TrayIconBuilder::new().menu(&menu).show_menu_on_left_click(true);
      if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
      }

      let toggle_dashboard_item = toggle_dashboard_i.clone();
      let rolling_view_item = rolling_view_i.clone();
      let fixed_view_item = fixed_view_i.clone();
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
            DASHBOARD_EDIT_MENU_ID => {
              let enabled = if let Some(state) = app.try_state::<Mutex<bool>>() {
                match state.lock() {
                  Ok(mut current) => {
                    *current = !*current;
                    *current
                  }
                  Err(_) => false,
                }
              } else {
                false
              };

              #[cfg(target_os = "macos")]
              {
                if let Some(window) = app.get_webview_window("main") {
                  if let Ok(ns_window) = window.ns_window() {
                    let ns_id = ns_window as id;
                    unsafe {
                      let level = if enabled { EDIT_WINDOW_LEVEL } else { DESKTOP_WINDOW_LEVEL };
                      let _: () = msg_send![ns_id, setLevel: level];
                    let _: () = msg_send![ns_id, setIgnoresMouseEvents: !enabled];
                    }
                    let _ = update_dashboard_edit_menu(&toggle_dashboard_item, enabled);
                    let _ = window.emit("dashboard-edit-mode-changed", enabled);
                  }
                }
              }
            }
            VIEW_ROLLING_MENU_ID | VIEW_FIXED_MENU_ID => {
              let next_mode = if event.id.as_ref() == VIEW_FIXED_MENU_ID {
                "fixed"
              } else {
                "rolling"
              };

              if let Ok(store) = app.store("tasks.json") {
                let mut data = store.get("tasksData").unwrap_or(Value::Null);
                set_saved_view_mode(&mut data, next_mode);
                store.set("tasksData", data.clone());
                let _ = store.save();
                let _ = update_view_mode_menus(&rolling_view_item, &fixed_view_item, next_mode);
                if let Some(window) = app.get_webview_window("main") {
                  let _ = window.emit("tasks-updated", data);
                }
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
