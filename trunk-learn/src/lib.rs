// use wasm_bindgen::prelude::*;
// use web_sys::window;

// fn start_app() {
//   let document = window().and_then(|win|win.document())
//     .expect("Could not access document");
//   let body = document.body().expect("Could not access document body");
//   let text_node = document.create_text_node("hello world!!!!!");
//   body.append_child(&text_node).expect("Failed to append text");
// }

// #[wasm_bindgen(start)]
// pub async fn run() {
//   console_error_panic_hook::set_once();
//   start_app();
// }
