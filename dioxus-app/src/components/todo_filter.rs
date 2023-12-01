use dioxus::prelude::*;
use crate::models::{Todos, Filter};

pub fn todo_filter(cx: Scope) -> Element {
    let todos = use_shared_state::<Todos>(cx)?;
    let todos_read = todos.read();

    let items_left = todos.read().items_left();
    let item_text = if items_left == 1 {
        "item left"
    } else {
        "items left"
    };

    let show_clear_completed = todos_read.show_clear_completed();

    cx.render(rsx! {
        (!todos_read.is_empty()).then(|| rsx! {
            footer {
                class: "footer",
                span { class: "todo-count", "{items_left} {item_text}" }
                ul { class: "filters",
                    filter_item { item: Filter::All }
                    filter_item { item: Filter::Active }
                    filter_item { item: Filter::Completed }
                }
                show_clear_completed.then(|| rsx! {
                  button {
                      class: "clear-completed",
                      onclick: move |_| {
                          todos.write().clear_completed();
                      },
                      "Clear completed",
                  }
              })
            }
        })
    })
}


#[inline_props]
fn filter_item(cx: Scope, item: Filter) -> Element {
    let filter = use_shared_state::<Filter>(&cx)?;

    let class = if *filter.read() == *item {
        "selected"
    } else {
        ""
    };

    let onclick = move |_| *filter.write() = *item;

    #[cfg(feature = "web")]
    {
        let href = match item {
            Filter::All => "#/",
            Filter::Active => "#/active",
            Filter::Completed => "#/completed",
        };

        render! {
            li {
                a { 
                  class: "{ class }", 
                  href: "{href}", 
                  onclick: onclick, 
                  "{item}"
                },
            }
        }
    }

    #[cfg(feature = "desktop")]
    render! {
        li {
            a { class: "{ class }", onclick: onclick, "{item}" },
        }
    }
}