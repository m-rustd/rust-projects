use std::{collections::BTreeMap, ops::{DerefMut, Deref}};

use serde::{Serialize, Deserialize};

use crate::platform::{get_store, StoreTrait};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todos {
    items: BTreeMap<u32, TodoItem>,
    next_id: u32,
}

impl Default for Todos {
    fn default() -> Self {
        Self { items: BTreeMap::new(), next_id: 1 }
    }
}

impl Deref for Todos {
  type Target = BTreeMap<u32, TodoItem>;

  fn deref(&self) -> &Self::Target {
      &self.items
  }
}

impl DerefMut for Todos {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.items
  }
}

impl Todos {
    pub fn create_todo(&mut self, title: impl Into<String>) {
        let id = self.next_id;
        self.next_id += 1;
        self.items.insert(
            id, 
            TodoItem { 
                id, 
                title: title.into(),
                completed: false 
            }
        );
        self.save();
    }

    // 获取过滤后的所有todo
    pub fn get_filtered_todos(&self, filter: &Filter) -> Vec<u32> {
        self.items.iter()
            .filter(|(_, todo)| match filter {
                Filter::All => true,
                Filter::Active => !todo.completed,
                Filter::Completed => todo.completed,
            })
            .map(|(id, _)| *id)
            .collect()
    }

    // 保存数据到本地
    pub fn save(&self) {
        let store = get_store();
        store.set(self);
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::All => write!(f, "All"),
            Filter::Active => write!(f, "Active"),
            Filter::Completed => write!(f, "Completed"),
        }
    }
}