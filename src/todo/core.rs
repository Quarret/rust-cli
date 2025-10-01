// src/todo/core.rs
// rust 模块默认private
pub struct TodoItem {
    pub title: String,
    pub content: String,
  }
  
  pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
      title: title.to_string(),
      content: content.to_string(),
    }
  }
  
  pub fn get_todo_list() -> Vec<TodoItem> {
    let mut todos: Vec<TodoItem> = Vec::new();
  
    todos.push(create_todo_item("learn rust", "read rust book"));
    todos.push(create_todo_item("work", "complete required"));
    todos.push(create_todo_item("play", "play game"));
  
    return todos;
  }