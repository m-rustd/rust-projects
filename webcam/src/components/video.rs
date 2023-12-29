use serde_json::json;
use sycamore::{futures::*, prelude::*};
use tracing::info;

use crate::{AppState, VideoStream, Controls};

#[component]
pub fn Video<G: Html>(ctx: Scope) -> View<G> {
    let state = use_context::<AppState>(ctx);
    // 类似computed
    let constraints = create_selector(ctx, || match state.device_id.get().as_str() {
        "" => json!({
            "facingMode": "user",
        }),
        id => json!({
            "deviceId": {
                "exact": id
            }
        }),
    });
    let show_controls = create_signal(ctx, true);
    let video_ref = create_node_ref(ctx);
    create_effect(ctx, move || {
        constraints.track();
        spawn_local_scoped(ctx, async move {
            info!("future spawned: {:?}", constraints.get());
            let el = video_ref.get::<DomNode>().unchecked_into();
            let video_stream = VideoStream::new(el);
            video_stream.set_video_src(&constraints.get()).await;
        });
    });

    view! {ctx,
        div(
            class="relative",
            on:mouseover = move |_| show_controls.set(true),
            on:mouseout = move |_| show_controls.set(false),
        ) {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width={state.get_width()},
                height={state.get_height()},
            )
            Controls(show_controls)
        }
    }
}