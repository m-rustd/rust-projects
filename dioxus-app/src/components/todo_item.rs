use dioxus::prelude::*;

use crate::models::Todos;

#[inline_props]
pub fn todo_item(cx: Scope, id: u32) -> Element {
    let todos = use_shared_state::<Todos>(&cx)?;
    let todos_read = todos.read();
    let todo = todos_read.get(id);
    
    cx.render(rsx! {
        div {
            "todo_item"
        }   
    })
}