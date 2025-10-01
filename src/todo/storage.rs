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

/// 将item序列化为json保存到文件中, 
/// 保存过程出错不奔溃返回错误信息,使用match实现, 错误的结束可以用 ? 运算符实现
// pub fn save_todo_list(save_file: &str, todos: &Vec<TodoItem>) {
//     match serde_json::to_string(todos) {
//       Ok(data) => match fs::write(save_file, data) {
//         Err(msg) => {
//           println!("save file error: {}", msg);
//         }
//         Ok(_) => {
//           println!("save file success");
//         }
//       },
//       Err(msg) => {
//         println!("save file error: {}", msg);
//       }
//     }
// }

/**
 * 序列化返回的结果是Result<String, error>
 * 需要同一结果为Result<(), String>
 * 匿名函数map_err(|e|, e.to_string())实现
 * ?错误传递
 * 如果 Result 是 Err，立即返回错误
 */
pub fn save_todo_list(save_file: &str, todos: &Vec<TodoItem>) -> Result<(), String> {
    let data = serde_json::to_string(todos).map_err(|e| e.to_string())?;
    fs::write(save_file, data).map_err(|e| e.to_string())?;
    Ok(())
}
