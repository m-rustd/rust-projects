mod controls;
mod video;

use sycamore::{prelude::*, futures::*};

pub use controls::Controls;
use tracing::info;
pub use video::Video;
use wasm_bindgen::{closure::Closure, UnwrapThrowExt, intern, JsCast};
use web_sys::Event;

use crate::{AppState, set_window_decorations};

#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    let state = AppState::new().await;
    provide_context(ctx, state);

    window_event_listener(
        ctx,
        "resize",
        Box::new(move |_| {
            let state = use_context::<AppState>(ctx);
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            state.dimensions.set((width, height));
            info!("Window resize: {}x{}", width, height);
        }),
    );

    window_event_listener(
        ctx,
        "mouseover",
        Box::new(move |_| {
          spawn_local_scoped(ctx, async move {
                info!("## window mouseover");
                // 调用tauri方法
                set_window_decorations(true).await;
            })
        }),
    );

    window_event_listener(
        ctx,
        "mouseout",
        Box::new(move |_| {
            spawn_local_scoped(ctx, async move {
              info!("** window mouseout");
              // 调用tauri方法
              set_window_decorations(false).await;
            });
        }),
    );

    view! { ctx,
        Video()
    }
}

fn window_event_listener<'a>(ctx: Scope<'a>, event: &str, callback: Box<dyn Fn(Event) + 'a>) {
    let window = web_sys::window().unwrap();
    let handler: Box<dyn Fn(Event) + 'static> = unsafe {
        std::mem::transmute(callback)
    };
    let callback = Closure::wrap(handler);
    window
        .add_event_listener_with_callback(intern(event), callback.as_ref().unchecked_ref())
        .unwrap_throw();
    
    on_cleanup(ctx, move || {
        drop(callback);
    });
}