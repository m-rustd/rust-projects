use dioxus::prelude::*;
use tracing::info;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    render! {
         h1 {
             "counter: {count}"
         }
         button {
             onclick: move |_| count.set(*count.current() + 1),
             "+"
         }
         button {
             onclick: move |_| count.set(*count.current() - 1),
             "-"
         }
    }
    // let count = use_state(cx, || 0);
    // render! {
    //      h1 {
    //          "counter: {count}"
    //      }
    //      button {
    //          onclick: move |_| count.set(**count + 1),
    //          "+"
    //      }
    //      button {
    //          onclick: move |_| count.set(**count - 1),
    //          "-"
    //      }
    // }
    // let mut count = use_state(cx, || 0);
    // render! {
    //     h1 {
    //         "counter: {count}"
    //     }
    //     button {
    //         onclick: move |_| count += 1,
    //         "+"
    //     }
    //     button {
    //         onclick: move |_| count -= 1,
    //         "-"
    //     }
    // }
}
//     render! {
//       div {
//           "This post has ",
//           b { "{props.score}" },
//           " likes"
//       }
//     }  
// }