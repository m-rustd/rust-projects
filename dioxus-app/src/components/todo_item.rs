use dioxus::{prelude::*, html::input_data::keyboard_types::Key};
use tracing::info;

use crate::models::Todos;

// #[derive(Props, PartialEq)]
// pub struct TodoItemProps {
//     pub id: u32,
// }

#[inline_props]
pub fn todo_item(cx: Scope, id: u32) -> Element {
    // let id = cx.props.id;
    let todos = use_shared_state::<Todos>(cx)?;
    let todos_read = todos.read();
    let todo = todos_read.get(&id)?;

    let is_editing = use_state(&cx, || false);
    let draft = use_state(&cx, || todo.title.clone());

    let completed = if todo.completed { "completed" } else { "" };
    let editing = if *is_editing.get() { "editing" } else { "" };
    
    cx.render(rsx! {
        li {
            class: "{completed} {editing}",
            div {
                class: "view",
                input {
                    class: "toggle",
                    r#type: "checkbox",
                    id: "todo-{todo.id}",
                    checked: "{todo.completed}",
                    onclick: move |e| {
                      info!("todo item clicked {e:?}");
                      todos.write().toggle_todo(*id);
                    }
                },
                label {
                    onclick: move |e| {
                        info!("label clicked {e:?}");
                        is_editing.set(true);
                    },
                    "{todo.title}"
                }
            }
            is_editing.then(|| rsx! {
                input {
                    class: "edit",
                    value: "{draft}",
                    oninput: move |e| {
                        info!("input changed {e:?}");
                        draft.set(e.value.clone());
                    },
                    autofocus: "true",
                    onkeypress: move |e| {
                        match e.key() {
                            Key::Enter | Key::Escape | Key::Tab => {
                                is_editing.set(false);
                                todos.write().update_todo(*id, draft.get());
                            },

                            _ => {}
                      }
                    },
                    onmouseout: move |e| {
                        info!("mouse out: {e:?}");
                        is_editing.set(false);
                        todos.write().update_todo(*id, draft.get());
                    }
                }
            })
        }   
    })
}