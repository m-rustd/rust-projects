pub mod components;
pub mod platform;
pub mod models;

use dioxus::prelude::*;
use models::{Filter, Todos};
use platform::{get_store, StoreTrait};
use tracing::info;

use crate::components::{todo_input, todo_filter, todo_item};

pub fn app(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || {
        let store = get_store();
        store.get()
    });
    use_shared_state_provider(&cx, Filter::default);

    let todos = use_shared_state::<Todos>(&cx)?;
    let filter = use_shared_state::<Filter>(cx)?;

    let filtered_todos = todos.read().get_filtered_todos(&filter.read());

    info!("filtered todos: {filtered_todos:?}");

    cx.render(rsx! {
        section {
            class: "todoapp",
            div {
                todo_input {}
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| {
                        rsx!(todo_item { key: "{id}", id: *id })
                    })
                }
                todo_filter{}
            }
        }
    })
}

