fn main() {
    // 存储 todoList 的标题和内容
    let mut inputs: Vec<String> = Vec::new();
    // args[0] 存储程序的exe文件地址
    let args: Vec<String> = std::env::args().collect();
    let mut ok = args[1].clone() == "create";
  
    while ok {
      let len = inputs.len();
  
      if len == 0 {
        println!("Please input todo title");
  
        let mut title = String::new();
  
        std::io::stdin()
          .read_line(&mut title)// 读到换行停止
          .expect("read line failed"); // 异常中断(?)
  
        if title.is_empty() {
          continue;
        }
        
        // trim() 去除字符串的头尾换行和空格
        inputs.push(title.trim().to_string());
  
      } else if len == 1 {
        println!("Please input todo content");
  
        let mut content = String::new();
  
        std::io::stdin()
          .read_line(&mut content)
          .expect("read line failed");
  
        if content.is_empty() {
          continue;
        }
  
        inputs.push(content.trim().to_string());
      }
  
      else {
        println!("title:   [{}]", inputs[0].clone());
        println!("content: [{}]", inputs[1].clone());
        println!("Are you sure to create this todo? (y/n)");
  
        let mut sure = String::new();
  
        std::io::stdin()
          .read_line(&mut sure)
          .expect("read line failed");
  
        if sure.trim().to_lowercase() != "n" {
          ok = false;
        }else{
          inputs.clear();
        }
      }
    }
  
    let title = inputs[0].clone();
    let content = inputs[1].clone();
  
    println!("create todo title: {}, content: {}", title, content);
  }