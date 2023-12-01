use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::models::Todos;

pub fn todo_input(cx: Scope) -> Element {
    let todos = use_shared_state::<Todos>(&cx)?;
    let draft = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        header {
            class: "header",
            h1 { "todos" }
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{draft}",
                oninput: move |e| {
                    draft.set(e.value.clone());
                },
                onkeydown: move |e| {
                    if e.key() == Key::Enter &&!draft.is_empty() {
                        todos.write().create_todo(draft.get());
                        draft.set("".to_string());
                    }
                }
            }
        }
    })
}
