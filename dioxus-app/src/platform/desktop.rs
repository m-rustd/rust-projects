use std::{path::PathBuf, fs::File, io::{Read, Write}, env};
use tracing::info;

use crate::models::Todos;

use super::StoreTrait;

const TODO_FILE: &str = "todos_dioxus.json";

#[derive(Debug)]
struct FileStore {
    path: PathBuf,
}

impl Default for FileStore {
    fn default() -> Self {
        let path = env::current_dir().unwrap().join(TODO_FILE);
        info!("desktop store: {:?}", path);

        Self {
            path,
        }
    }
}

impl StoreTrait for FileStore {
    fn get(&self) -> Todos {
        if let Ok(mut file) = File::open(&self.path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn set(&self, items: &Todos) {
        let content = serde_json::to_string(items).unwrap();
        let mut file = File::create(&self.path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}

pub fn get_store() -> impl StoreTrait {
    FileStore::default()
}