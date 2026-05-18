use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
    pub priority: u8,
    pub created_at: DateTime<Local>,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            priority: 1, // Default 1.
            created_at: Local::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: u32,
}

impl TodoList {
    pub fn add(&mut self, title: String) -> &Todo {
        let todo = Todo::new(self.next_id, title);
        self.next_id += 1;
        self.todos.push(todo);
        self.todos.last().unwrap()
    }

    pub fn remove(&mut self, id: u32) -> bool {
        // let index = self.todos.iter().position(|todo| todo.id == id);
        // index.map(|i| self.todos.remove(i))
        let before = self.todos.len();
        self.todos.retain(|t| t.id != id);
        return before != self.todos.len();
    }

    pub fn completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            return true;
        } else {
            return false;
        }
    }

    pub fn list(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn prioritize(&mut self, id: u32, priority: u8) -> bool {
        // self.todos.sort_by(|a, b| b.priority.cmp(&a.priority));
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.priority = priority;
            return true;
        } else {
            return false;
        }
    }
}
