// src/todo/storage.rs
use super::core::TodoItem;
use std::fs;

pub fn read_todo_list(save_file: &str) -> Vec<TodoItem> {
  let mut result: Vec<TodoItem> = Vec::new();

    // 1. 读取文件
  match fs::read_to_string(save_file) {
    /* 文件读取成功，content 是文件内容 */ 
    Ok(content) => match serde_json::from_str(content.as_str()) {
        /* JSON 解析成功 */
      Ok(mut list) => result.append(&mut list),
      _ => {
        println!("parse file error");
      }
    },
    _ => {
      println!("read file error");
    }
  }

  // 如果没有读取到任何数据, 提供默认示例
  if result.len() == 0 {
    result.push(TodoItem::new("learn rust", "read rust book"));
    result.push(TodoItem::new("work", "complete required"));
    result.push(TodoItem::new("play", "play game"));
  }

  return result;
}

pub fn save_todo_list(save_file: &str, todos: &Vec<TodoItem>) {
    // unwrap() 的作用：
    // - 如果 Result 是 Ok，取出值
    // - 如果 Result 是 Err，程序 panic（崩溃）

    let data = serde_json::to_string(todos).unwrap();
    // 如果文件不存在，会自动创建
    // 如果文件存在，会完全覆盖
    fs::write(save_file, data).unwrap();
}