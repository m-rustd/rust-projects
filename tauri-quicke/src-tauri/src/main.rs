// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{EventType, Button};
use serde::Serialize;
use tauri::{Manager, Window, Position, LogicalPosition};

// 默认false， 阻止事件冒泡
// static mut IS_SIMULATE: bool = false; 
static mut MOUSE_POSITION: (f64, f64) = (0.0, 0.0); 

#[derive(Serialize, Clone)]
struct ButtonPayload {
    button: String,
    x: f64,
    y: f64,
}

fn get_button_name(button: &Button) -> String {
    return match button {
        Button::Left => "Left".to_string(),
        Button::Right =>  "Right".to_string(),
        Button::Middle => "Middle".to_string(),
        Button::Unknown(_) => "Unknown".to_string(),
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn window_show(window: Window, show: bool, x: Option<f64>, y: Option<f64>) {
    // println!("window_show---{}----x: {} y: {}", show, x, y);
    if show {
      let (window_width, window_height) = (300.0, 300.0);
      let mut new_x = x.unwrap();
      let mut new_y = y.unwrap();
      if new_x <= window_width/2.0 {
          new_x = window_width/2.0;
      }
      if new_y <= window_height/2.0 {
          new_y = window_height/2.0;
      }
      println!("x: {} y: {}", new_x, new_y);
      let position = LogicalPosition{ x: new_x - window_width / 2.0, y: new_y - window_height / 2.0 };
      window.show().unwrap();
      window.set_position(Position::Logical(position) ).unwrap();
    } else {
      window.hide().unwrap();
    }
}

fn event_listener(main_window: Window) {
    // 阻止鼠标事件
    rdev::grab(move |event| {
        let is_block = match event.event_type {
          EventType::ButtonPress(button) => {
              match button {
                  Button::Right => {
                      unsafe {
                          main_window.emit("buttonDown", ButtonPayload {
                              button: get_button_name(&button),
                              x: MOUSE_POSITION.0,
                              y: MOUSE_POSITION.1
                          }).unwrap();
                      }
                      true
                  },
                  _ => { 
                    false
                  }
              }
          },
          EventType::ButtonRelease(button) => {
              match button {
                  Button::Right => {
                      // main_window.hide().unwrap();
                      main_window.emit("buttonUp", ButtonPayload {
                          button: get_button_name(&button),
                          x: 0.0,
                          y: 0.1
                      }).unwrap();
                      true
                  },
                  _ => { false }
              }
          },
          EventType::MouseMove { x, y } => {
              unsafe {
                  MOUSE_POSITION = (x, y);
                  false
              }
          },
          _ => { false }
      };
      if is_block {
          None
      } else {
          Some(event)
      }
    }).unwrap();
}

fn main() {
    let builder = tauri::Builder::default()
        .setup(|app|{
            let main_window = app.get_window("main").unwrap();
            main_window.open_devtools();
            main_window.hide().unwrap();
            tauri::async_runtime::spawn(async move {
                event_listener(main_window);
            });
            Ok(())
        });
    builder.invoke_handler(tauri::generate_handler![
        greet, 
        window_show
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}