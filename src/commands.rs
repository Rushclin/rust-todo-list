use crate::storage;

pub fn add(title: String) {
    let mut list = load_or_exit();
    let todo = list.add(title);
    println!("✅ Task added  [#{}] {}", todo.id, todo.title);
    save_or_exit(&list);
}

pub fn list(all: bool) {
    let list = load_or_exit();
    let todos = list.list();

    if todos.is_empty() {
        println!("📭 No tasks found. Add one with: todo add \"My task\"");
        return;
    }

    let filtered: Vec<_> = if all {
        todos.iter().collect()
    } else {
        todos.iter().filter(|t| !t.completed).collect()
    };

    if filtered.is_empty() {
        println!("🎉 All tasks are completed! (use --all to see everything)");
        return;
    }

    println!("{:<5} {:<8} {}", "ID", "STATUS", "TITLE");
    println!("{}", "-".repeat(40));

    for todo in filtered {
        let status = if todo.completed { "✅ done " } else { "⬜ todo " };
        let title  = if todo.completed {
            format!("\x1b[2m{}\x1b[0m", todo.title) // greyed out if completed
        } else {
            todo.title.clone()
        };
        println!("#{:<4} {} {}", todo.id, status, title);
    }
}

pub fn complete(id: u32) {
    let mut list = load_or_exit();
    if list.completed(id) {
        println!("✅ Task #{id} marked as completed!");
        save_or_exit(&list);
    } else {
        eprintln!("❌ No task found with ID #{id}");
        std::process::exit(1);
    }
}

pub fn remove(id: u32) {
    let mut list = load_or_exit();
    if list.remove(id) {
        println!("🗑️  Task #{id} deleted.");
        save_or_exit(&list);
    } else {
        eprintln!("❌ No task found with ID #{id}");
        std::process::exit(1);
    }
}

pub fn clear_done() {
    let mut list = load_or_exit();
    let before = list.todos.len();
    list.todos.retain(|t| !t.completed);
    let removed = before - list.todos.len();
    save_or_exit(&list);
    println!("🧹 {removed} completed task(s) deleted.");
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn load_or_exit() -> crate::todo::TodoList {
    storage::load().unwrap_or_else(|e| {
        eprintln!("❌ {e}");
        std::process::exit(1);
    })
}

fn save_or_exit(list: &crate::todo::TodoList) {
    storage::save(list).unwrap_or_else(|e| {
        eprintln!("❌ {e}");
        std::process::exit(1);
    });
}