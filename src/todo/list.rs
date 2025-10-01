// src/todo/list.rs
/*
super, 表示当前模块的父模块。
self, 表示当前模块自身。
crate, 表示当前根模块, 即 src 目录, 如果是第三方库，则替换为库名称。
*/
use super::core::TodoItem;

pub fn list_todo(todos: &Vec<TodoItem>) {
  for todo in todos {
    println!("todo title: {}, content: {}", todo.title, todo.content);
  }
}