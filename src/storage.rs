use std::fs;
use std::path::PathBuf;

use crate::todo::TodoList;

/// Get datas path for JSON (~/.local/share/todo-cli/todos.json)
pub fn data_path() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("todo-cli");
    fs::create_dir_all(&dir).expect("Impossible to create datas dir");
    dir.join("todos.json")
}

pub fn load() -> Result<TodoList, String> {
    let path = data_path();

    if !path.exists() {
        return Ok(TodoList::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Impossible to read JSON file : {e}"))?;

    serde_json::from_str(&content)
        .map_err(|e| format!(
            "Invalid JSON file ({path:?}) : {e}\n\
             DISCLEAMER : manual delete or update file content."
        ))
}

pub fn save(list: &TodoList) -> Result<(), String> {
    let path = data_path();
    let json = serde_json::to_string_pretty(list)
        .map_err(|e| format!("Erreur of serialization JSON: {e}"))?;
    fs::write(&path, json)
        .map_err(|e| format!("Impossible to write JSON file: {e}"))
}