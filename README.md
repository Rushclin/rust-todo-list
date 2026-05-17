# Todo CLI — A Command-Line Task Manager in Rust

A fast, lightweight, and persistent command-line todo application built with Rust.
Tasks are stored locally in a JSON file on your machine — no internet connection required.

---

##  Project Structure

```
todo-cli/
├── Cargo.toml          # Project dependencies and metadata
└── src/
    ├── main.rs         # Entry point — CLI definition with clap
    ├── todo.rs         # Data model (Todo, TodoList structs)
    ├── storage.rs      # JSON file read/write logic
    └── commands.rs     # Business logic for each command
```

---

##  How it works

### Data model (`todo.rs`)
Each task is a `Todo` struct with:
- `id` — unique auto-incremented identifier
- `title` — the task description
- `completed` — boolean completion status
- `created_at` — timestamp of creation (via `chrono`)

All tasks are wrapped in a `TodoList` struct which also tracks `next_id` to ensure IDs are never reused, even after deletions.

### Persistence (`storage.rs`)
Tasks are saved as a pretty-printed JSON file located at:

| OS      | Path                                              |
|---------|---------------------------------------------------|
| Linux   | `~/.local/share/todo-cli/todos.json`              |
| macOS   | `~/Library/Application Support/todo-cli/todos.json` |
| Windows | `C:\Users\<you>\AppData\Local\todo-cli\todos.json` |

The directory is created automatically on first run.
If the file does not exist yet, the app starts with an empty list.
If the file is corrupted or malformed, a clear error message is shown with a hint to fix or delete the file manually.

### CLI parsing (`main.rs`)
Commands are parsed using [`clap`](https://docs.rs/clap) with the `derive` feature.
Running `todo --help` or `todo <command> --help` always shows up-to-date usage information.

### Command logic (`commands.rs`)
Each command loads the JSON file, performs its operation in memory, then saves the result back to disk. Errors (bad ID, write failure, etc.) exit with code `1` and print a clear message to `stderr`.

---

## Prerequisites

Make sure you have **Rust** installed. If not, install it via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your installation:

```bash
rustc --version
cargo --version
```

You need at least **Rust 1.70+**.

---

##  Getting started

### 1. Clone the repository

```bash
git clone https://github.com/Rushclin/todo-cli.git
cd todo-cli
```

### 2. Build the project

```bash
cargo build --release
```

The compiled binary will be available at:

```
./target/release/todo        # Linux / macOS
./target/release/todo.exe    # Windows
```

### 3. (Optional) Install globally

To use `todo` from anywhere in your terminal:

```bash
cargo install --path .
```

Then verify it works:

```bash
todo --version
```

If ```todo``` command not work, try to use ```todo-cli```

---

## 📦 Dependencies

Defined in `Cargo.toml`:

| Crate        | Version | Purpose                              |
|--------------|---------|--------------------------------------|
| `clap`       | 4       | CLI argument parsing (derive macros) |
| `serde`      | 1       | Serialization / deserialization      |
| `serde_json` | 1       | JSON format support                  |
| `dirs`       | 5       | Cross-platform config/data paths     |
| `chrono`     | 0.4     | Date and time with serde support     |

---

## 💻 All Available Commands

### ➕ Add a task

```bash
todo add "Your task title"
```

Example:

```bash
todo add "Learn Rust"
todo add "Read the clap documentation"
todo add "Finish the todo project"
```

Output:

```
✅ Task added  [#0] Learn Rust
```

---

### 📋 List tasks

List only **pending** tasks (default):

```bash
todo list
```

List **all** tasks, including completed ones:

```bash
todo list --all
# or shorthand:
todo list -a
```

Output:

```
ID    STATUS   TITLE
----------------------------------------
#0    ⬜ todo  Learn Rust
#1    ⬜ todo  Read the clap documentation
#2    ✅ completed  Finish the todo project
```

When there are no tasks:

```
📭 No tasks found. Add one with: todo add "My task"
```

When all tasks are completed:

```
🎉 All tasks are completed! (use --all to see everything)
```

---

###  Mark a task as completed

```bash
todo completed <ID>
```

Example:

```bash
todo completed 0
```

Output:

```
✅ Task #0 marked as completed!
```

If the ID does not exist:

```
❌ No task found with ID #99
```

---

### Delete a task

```bash
todo remove <ID>
```

Example:

```bash
todo remove 1
```

Output:

```
🗑️  Task #1 deleted.
```

---

### 🧹 Clear all completed tasks

Remove every task that has been marked as completed in one shot:

```bash
todo clear
```

Output:

```
🧹 2 completed task(s) deleted.
```

---

###  Help

Global help:

```bash
todo --help
```

Help for a specific command:

```bash
todo add --help
todo list --help
todo completed --help
todo remove --help
todo clear --help
```

---

## Running Without Installing

You can run any command directly with `cargo run` during development:

```bash
cargo run -- add "My task"
cargo run -- list
cargo run -- list --all
cargo run -- completed 0
cargo run -- remove 1
cargo run -- clear
```

> The `--` separates Cargo's own arguments from your program's arguments.

---

## JSON File Example

After adding a few tasks, your `todos.json` file will look like this:

```json
{
  "todos": [
    {
      "id": 0,
      "title": "Learn Rust",
      "completed": true,
      "created_at": "2026-05-17T10:32:00.123+01:00"
    },
    {
      "id": 1,
      "title": "Read the clap documentation",
      "completed": false,
      "created_at": "2026-05-17T10:33:10.456+01:00"
    }
  ],
  "next_id": 2
}
```

You can edit this file manually if needed. If the file becomes invalid JSON, the app will refuse to run and display:

```
❌ Invalid JSON file (...path...): <error detail>
   Tip: delete or fix the file manually.
```

---

## Possible extensions

Here are ideas to extend this project once the basics are working:

- **Priorities** — add a `priority` field (`low`, `medium`, `high`) and sort by it
- **Due dates** — add a `due_date` field and warn when a task is overdue
- **Categories / tags** — group tasks by project or label
- **Search** — `todo search "keyword"` to filter by title
- **TOML format** — swap `serde_json` for the `toml` crate as an alternative storage format
- **Interactive TUI** — build a terminal UI with [`ratatui`](https://github.com/ratatui-org/ratatui) and `crossterm`

---

## License

MIT — feel free to use, modify, and distribute.