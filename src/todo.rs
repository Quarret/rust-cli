// src/todo.rs
pub mod core;
pub mod create;
/*
我们可以使用 mod 关键字来声明子模块。同样的, 子模块需要 pub 关键字修饰来公开给外部访问。
在 Rust 中, 每个模块都有一个 mod.rs 文件, 该文件是模块的入口文件。 在 mod.rs 文件中, 我们可以定义模块的公开内容, 如结构体、函数、模块等。
如果使用的 rustc 版本在 1.30 以前, 那么这是唯一声明模块入口的方法。
但如果实在 1.30 以后, 那么可以在模块同级位置创建一个同名的 .rs 文件来作为模块入口声明。
这里使用的就是使用与模块同名的 .rs 文件作为模块入口声明。
需要注意的是, 模块是可以直接声明的, 而无需单独文件。
*/
pub mod list {
    use super::core::TodoItem;
  
    pub fn list_todo(todos: &Vec<TodoItem>) {
      for todo in todos {
        println!("todo title: {}, content: {}", todo.title, todo.content);
      }
    }
  }