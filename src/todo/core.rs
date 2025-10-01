// src/todo/core.rs
use serde::{Deserialize, Serialize};
use serde_json;
use clap::Subcommand;

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
/// Create a new todo item
Create {
    // #[arg(short, long)] 告诉 clap 该字段对应一个参数, 并指定参数的短名称和长名称。
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    content: Option<String>,
},
/// List all todo items
List {
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    content: Option<String>,
},
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

pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

impl TodoItem {
    // self == this
    // Self == 当前类
    pub fn new(title: &str, content: &str) -> Self {
      create_todo_item(title, content)
    }
  
    // pub fn serializer(&self) -> String {
    //   serde_json::to_string(self).unwrap()
    // }
  
    // pub fn deserializer(s: &str) -> Self {
    //   serde_json::from_str(s).unwrap()
    // }
}

// 默认实现替代手动指定类
pub trait Serializer
where
// 单纯self不满足serial和deserial的约束, 要使用 where ... + ... 实现约束
// 编译器不能推导生命周期, 约束不符合deserial
    Self: Sized + Serialize + for<'a> Deserialize<'a>,
{
  fn serialize(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

  fn deserialize<S: Into<String>>(s: S) -> Self {
    serde_json::from_str(&s.into()).unwrap()
  }
}

impl Serializer for TodoItem {}



pub fn get_todo_list() -> Vec<TodoItem> {
    let mut todos: Vec<TodoItem> = Vec::new();

    todos.push(TodoItem::new("learn rust", "read rust book"));
    todos.push(TodoItem::new("work", "complete required"));
    todos.push(TodoItem::new("play", "play game"));

    return todos;
}



/*
条件测试
#[cfg(all(target_os="windows", test))] 表示仅在编译对象为 windows 平台时运行 cargo test 编译。
#[cfg(all(any(target_os = "ios", target_os = "android"), test))] 表示仅在编译对象为移动端时运行 cargo test 编译。
#[cfg(all(not(any(target_os = "ios", target_os = "android")), test))] 表示仅在编译对象非移动端时运行 cargo test 编译。
*/
#[cfg(test)]
mod tests {
  // 因为是子模块, 因此需要使用 super 关键字来引用父模块
  use super::{Serializer, TodoItem};

  /*
    assert!(expr): 如果 expr 为假, 抛出异常。
    assert_eq!(left, right): 如果 left 不等于 right, 抛出异常。
    assert_ne!(left, right): 如果 left 等于 right, 抛出异常。
    如果加上 debug_ 前缀, 则只在 Debug 模式运行, 如 debug_assert!(expr)。
   */

  #[test]
  fn test_todo_item_creation() {
    let item = TodoItem::new("test", "content");
    assert_eq!(item.title, "test");
    assert_eq!(item.content, "content");
  }

  #[test]
  fn test_serialization_roundtrip() {
    let original = TodoItem::new("test", "content");
    let serialized = original.serialize();
    let deserialized = TodoItem::deserialize(serialized);

    assert_eq!(original.title, deserialized.title);
    assert_eq!(original.content, deserialized.content);
  }
}