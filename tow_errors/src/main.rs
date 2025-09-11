// *************************** 高阶篇 ***************************


// ------------------- 1、异常处理 ------------------- 
// fn main() {
//     // 1）简单的panic
//     // panic!("程序崩溃了!");  // 程序会终止，并输出


//     divide(10, 0);
    
// }


// // 2）自定义panic, 在程序中按照条件触发
//  fn divide(x: i32, y: i32) -> i32 {
//     if y == 0 {
//         panic!("不能➗0!");
//     }
//     x / y // 返回值
//  }


// 3）配置panic，提高性能，但不清理资源
// 在 Cargo.toml 中设置 [profile.release] panic = 'abort'


// 4) 捕捉 panic 
// 导入 std::panic::catch_unwind 用于捕捉异常，但不推荐，rust鼓励显示错误处理


// 5）可恢复错误：Result 与 Option
// Rust不适用异常，而是返回枚举类型

// // Option: 表示可能为空的值。Some(T) or None
// fn find_char(s: &str, c: char) -> Option<usize> {
//     for (i, ch) in s.chars().enumerate() {
//         if ch == c {
//             return Some(i);
//         }
//     }
//     None
// }

// fn main() {
//     // match 处理可能的值。Option 常用于可能失败但无具体错误信息的场景。
//     match find_char("hello", 'w') {
//         // match遍历调用的时候，会判断返回值是Some(i)还是None
//         // 根据返回值，执行对应的表达式
//         Some(pos) => println!("找到位置：{}", pos),
//         None => println!("未找到"),
//     }
// }


// // Result<T, E>: 表示成功或失败。Ok(T) or Err(E)
// use std::fs::File;  // 导入File文件处理模块
// use std::io::{self, Read};  // 导入Read读io模块

// fn read_file(filename: &str) -> Result<String, io::Error> {
//     /*
//         参数: 文件名字符串
//         返回值: Result<字符串, io模块的异常Error>
//     */ 
//     // 
//     // ? 若出现错误，会提前返回;
//     let mut file = File::open(filename)?;  // 打开文件
//     let mut contents = String::new();  // 文件内容字符串对象
//     // 读取文件并转为字符串并传给contents变量
//     // ? 若出现错误，会提前返回;
//     let _ = file.read_to_string(&mut contents)?;
//     Ok(contents)  // 返回Result成功，并返回字符串内容

//     // // ? 等价于
//     // let mut file = match File::open(filename){
//     //     Ok(f) => f,
//     //     Err(e) => return Err(e),
//     // };
// }

// fn main() {
//     match read_file("hello.txt") {
//         // Result 的 E 是错误类型，这里是 io::Error。
//         // 成功返回 Ok(值)，失败返回 Err(错误)。
//         Ok(content) => println!("文件内容：{content}"),
//         Err(e) => println!("读取失败: {e}"),
//     }
// }


// 6）自定义错误
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError{
    Io(std::io::Error),
    Parse(ParseIntError),
    Custom(String),  // 自定义错误信息
}

/*
    枚举包装不同错误源。
    实现 Display 和 Debug 以打印。
    实现 From 以支持 ? 的自动转换。
    这允许统一处理多种错误。
*/ 
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO 错误: {}", e),
            MyError::Parse(e) => write!(f, "解析错误: {e}"),
            MyError::Custom(e) => write!(f, "自定义错误: {e}"),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError{
    fn from(err: ParseIntError) -> MyError {
        MyError::Parse(err)
    }
}

fn parse_number(s: &str) -> Result<i32, MyError> {
    // 手动转换或用From
    let num: i32 = s.parse().map_err(MyError::Parse)?;
    if num < 0 {
        return Err(MyError::Custom("负数无效".to_string()));
    }
    Ok(num)
}

fn main(){
    match parse_number("-5") {
        Ok(n) => println!("number: {n}"),
        Err(e) => println!("error: {e}")
    }
}

// ------------------- 总结：异常处理 ------------------- 
/*
    - 使用Result而非panic: 除非确实不可恢复
    - 早失败，早返回: 使用 ? 保持代码简洁
    - 提供上下文: 在Err中添加信息, 如使用anyhow::Context
    - 标准库 vs crate
        - 简单项目: 用std::io::Error
        - 复杂项目: 推荐 anyhow(用户友好错误) 或 thiserror(自定义错误宏)
    - 测试错误: 用 #[should_panic] 测试 panic 或 匹配Result::Err
    - 性能: Result零成本抽象，不会影响运行时，除非错误发生
    - 常见陷阱: 
        - 忘记处理Result, 导致编译错误(Rust强制处理)
        - 过度unwrap: 用在原型中，但生产代码中笔辩
        - 错误类型不兼容: 确保实现From或手动map_err; 
*/ 



// ------------------- HOMEWORK ------------------- 
/*
    - 编写一个函数读取文件并解析为整数列表，使用自定义错误处理解析失败。
    - 修改示例，使用 match 处理多级错误链。
    - 探索 std::error::Error trait 以创建更通用的错误。
*/ 
