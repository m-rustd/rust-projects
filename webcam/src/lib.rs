mod components;
mod model;

pub use components::*;
pub use model::*;

use sycamore::reactive::{RcSignal, create_rc_signal};
use wasm_bindgen::prelude::*;

pub struct AppState {
    pub dimensions: RcSignal<(u32, u32)>,
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            dimensions: create_rc_signal((800, 450)),
            device_id: create_rc_signal("".into()),
            devices: create_rc_signal(Devices::load().await),
        }
    }

    pub fn get_width(&self) -> u32 {
        self.dimensions.get().0
    }

    pub fn get_height(&self) -> u32 {
        self.dimensions.get().1
    }
}

// 绑定js方法
#[wasm_bindgen(module = "/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = "invokeSetWindowDecorations")]
    pub async fn set_window_decorations(decorations: bool);
}