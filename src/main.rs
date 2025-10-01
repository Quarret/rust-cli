//src/main.rs
use crate::todo::storage::{read_todo_list, save_todo_list};
use clap::Parser;
use todo::core::TodoCommand;

mod todo;

#[derive(Debug, Parser)]

// clap 自动生成 --version 和 --help 两个参数。
#[command(version, about, long_about = "Todo Cli")]
struct Program {
  // 告诉 clap 该字段对应一个子命令  
  #[command(subcommand)]
  
  pub command: TodoCommand,
}

fn main() {
  let args = Program::parse();
  let save_file = "todo.json";
  let mut todos = read_todo_list(save_file);

  match args.command {
    TodoCommand::Create => todo::create::create_todo(&mut todos),
    TodoCommand::List => todo::list::list_todo(&todos),
  }

  save_todo_list(save_file, &todos);
}