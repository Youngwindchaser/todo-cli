# 📝 Rust CLI Todo App

A minimalist command-line Todo application written in Rust. Manage your daily tasks using the terminal — add, list, and remove todos with persistent JSON storage.

## 🚀 Features

- ✅ Add new tasks from the command line
- 📋 List all saved tasks with index
- ❌ Remove tasks by number
- 💾 Persists tasks in `todo.json`
- ⚡ Built using `clap`, `serde`, and `serde_json`

## 🧱 Tech Stack

- **Rust**
- **clap** – for CLI parsing
- **serde & serde_json** – for JSON file read/write
- **cargo** – Rust's build system

## 📦 Installation

Clone the repo and build with Cargo:

```bash
git clone https://github.com/Youngwindchaser/todo-cli.git
cd todo-cli
cargo build
