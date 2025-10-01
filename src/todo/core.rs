// src/todo/core.rs
use serde::{Deserialize, Serialize};
use serde_json;
use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
/// Create a new todo item
Create,
/// List all todo items
List,
}

// rust 模块默认private
#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub title: String,
    pub content: String,
}

/*
new 方法: 用于创建一个新的 TodoItem, 现在可以直接用 TodoItem::new(...) 替代之前的 create_todo_item(...)。
serializer 方法: 将当前实例转换为 JSON 字符串。
deserializer 方法: 从 JSON 字符串还原为 TodoItem 实例。
*/
impl TodoItem {
    // self == this
    // Self == 当前类
    pub fn new(title: &str, content: &str) -> Self {
      create_todo_item(title, content)
    }
  
    pub fn serializer(&self) -> String {
      serde_json::to_string(self).unwrap()
    }
  
    pub fn deserializer(s: &str) -> Self {
      serde_json::from_str(s).unwrap()
    }
}
  
pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}
  
pub fn get_todo_list() -> Vec<TodoItem> {
    let mut todos: Vec<TodoItem> = Vec::new();

    todos.push(TodoItem::new("learn rust", "read rust book"));
    todos.push(TodoItem::new("work", "complete required"));
    todos.push(TodoItem::new("play", "play game"));

    return todos;
}