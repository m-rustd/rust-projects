use std::ops::Deref;

use tracing::info;
use web_sys::Storage;

use crate::models::Todos;

use super::StoreTrait;

const TODO_KEY: &str = "todos_dioxus";

pub struct LocalStorage(Storage);

impl Deref for LocalStorage {
    type Target = Storage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for LocalStorage {
    fn default() -> Self {
        Self(
            web_sys::window()
                .unwrap()
               .local_storage()
               .unwrap()
               .expect("no global local storage exists")
        )
    }
}

impl StoreTrait for LocalStorage {
    fn get(&self) -> Todos {
        if let Ok(Some(content)) = self.get_item(TODO_KEY) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Todos::default()
        }
    }

    fn set(&self, items: &Todos) {
        let content = serde_json::to_string(items).unwrap();
        self.set_item(TODO_KEY, &content).unwrap();
    }
}

pub fn get_store() -> impl StoreTrait {
    LocalStorage::default()
}