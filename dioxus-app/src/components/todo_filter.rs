use dioxus::prelude::*;

pub fn todo_filter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "todo_filter"
        }   
    })
}