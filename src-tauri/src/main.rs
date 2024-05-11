#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{ContextMenu, Menu, MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{async_runtime, AppHandle, Manager, Window};
use tokio::time::{sleep, Duration};

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn context_menu(app: AppHandle, window: Window) {
  let menu = MenuBuilder::new(&app)
    .items(&[
      &MenuItemBuilder::with_id("item1", "item1")
        .build(&app)
        .unwrap(),
      &MenuItemBuilder::with_id("item2", "item2")
        .build(&app)
        .unwrap(),
    ])
    .build()
    .unwrap();

  window.on_menu_event(move |_, _| {
    async_runtime::spawn(async move {
      sleep(Duration::from_secs(1)).await;
      println!("context menu event");
    });
  });

  menu.popup(window).unwrap();
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app = app.handle();
      let menu = Menu::new(app)?;
      menu.append(
        &SubmenuBuilder::new(app, "main-menu")
          .items(&[
            &MenuItemBuilder::with_id("sub-item1", "sub-item1")
              .build(app)
              .unwrap(),
            &MenuItemBuilder::with_id("sub-item2", "sub-item2")
              .build(app)
              .unwrap(),
          ])
          .build()
          .unwrap(),
      )?;

      let window = app.get_webview_window("main").unwrap();
      window.on_menu_event(|_, event| {
        async_runtime::spawn(async move {
          sleep(Duration::from_secs(1)).await;
          println!("menu event: {:?}", event);
        });
      });

      window.set_menu(menu).unwrap();

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet, context_menu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
