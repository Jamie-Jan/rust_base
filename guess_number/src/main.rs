// ---------------- 1、基础语法 ----------------  
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
    
//     // 一 变量

//     // 不可变
//     let x = 5;
//     // 格式化输出
//     println!("x 的值为: {}", x);

//     // 可变
//     let mut y = 5;
//     println!("y 的值为: {}", y);
//     y = 9527;
//     println!("y 的值为: {}", y);

//     // 常量 Constants 
//     // const 变量名: 类型 = 值;
//     println!("三小时的秒数为：{}", THREE_HOURS_IN_SECONDS);

//     // 变量遮蔽: 允许内层用相同的变量名
//     let z = 100;
//     let z = z + 1;  // 遮蔽，z 是 101

//     // 内作用域遮蔽：变量作用域，块级使用 {}, 超出作用域后则变量会被销毁
//     {
//         let z = z * 2; 
//         println!("内层 z 的值为: {}", z);  // 202
//     }

//     println!("外层 z 的值为: {}", z);  // 101


//     // 二、基本类型
//     // 1）标量类型
//     // integers i:有符号 u:无符号
//     let num:i8 = -10; 
//     let num2:u8 = 100;
//     println!("num: {}, nu2: {}", num, num2);  

//     // float
//     let float_num = 2.0;  // default: f64
//     let float_num2: f32 = 3.0;  
//     println!("float_f64: {}, float_f32: {}", float_num, float_num2);  

//     // bool
//     let t = true;
//     let _f = false;
//     if t {
//         println!("right");
//     }

//     // char:  Unicode 标量值
//     let c = 'z';  // 字符
//     let c_z: char = 'ℤ';
//     let heart_eyed_cat = '😻';
//     println!("字符：{}, {}, {}", c, c_z, heart_eyed_cat);


//     // 2) 复合类型
//     // tuples: 不可变
//     let tup: (i32, f64, u8) = (500, 2.1, 9);
//     let (x1, y1, z1) = tup;  // 解构赋值
//     println!("y1 的值为：{}", y1);
//     let five_hundred = tup.0;  // 根据索引取值
//     println!("five_hundred 的值为：{}", five_hundred);

//     // arrays: 固定长度，不可变;
//     // 变量名: [类型; 长度]
//     let a_1: [i32; 5] = [1, 2, 3, 4, 5];
//     let a_2 = [3; 5];  // 5个值都是3
//     println!("a_1 的值为：{}, a_2 的值为：{}", a_1[0], a_2[2]);


//     // 3) 函数 Functions
//     another_func();  // 无参函数调用
//     have_args_func(9527, 'J');  // 有参函数调用
//     let return_value = return_val(33);
//     println!("return_value的值为: {return_value}");

//     // 条件语句
//     let score = 85;
//     if score >= 90 {
//         println!("优秀!")
//     } else if score >= 80 {
//         println!("良好！")
//     } else if score >= 60{
//         println!("及格！")
//     } else {
//         println!("不及格！")
//     }

//     // build_user("jamie@.com", "Jamie");

// }

// // 无参函数定义
// fn another_func() {
//     println!("This is a another_func function!")
// }

// // 有参函数定义
// fn have_args_func(val: i32, label: char) {
//     println!("val的值为：{val}, label的值为：{label}");
// }

// // 有返回值
// fn return_val(x: i32) -> i32 {
//     x + 1 // 无分号，等于返回值;
// }





// ---------------- 2、结构体 ----------------  
// 定义结构体
// 1) 普通结构体
// struct User {
//     active: bool, 
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// // 2）元组结构体
// struct Color(i32, i32, i32);

// 3）单元结构体（用于不需要数据的类型）
// struct AlwaysEqual;
// let subject = AlwaysEqual;


// 实例化结构体
// let user1 = User{
//     active: true,
//     username: String::from("jamie"),
//     email: String::from("jamie@.com"),
//     sign_in_count: 1
// }
// 简写
// fn build_user(email: String, username: String) -> User{
//     User { 
//         active: true,
//         email, 
//         username,  
//         sign_in_count: 1
//     }
// }


// 定义与实例化
// #[derive(Debug)] // 启动调试打印
// #[derive(Debug)]
// struct Rectangle{
//     width: u32,
//     height: u32,
// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// // 方法（与结构体关联的函数，使用imp定义）
// impl Rectangle{
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     // 可变性 和 额外参数
//     //   &mut self: 允许修改实例
//     //   示例（额外参数）
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     // 关联函数：常用于构造函数, 使用 :: 调用
//     fn square(size: u32) -> Self {
//         Self { width: size, height: size}
//     }
// }

// fn main() {
//     let rect1 = Rectangle{
//         width: 30,
//         height: 50,
//     };
//     println!("rect1 是 {:?}", rect1);  // 使用 {:?} 打印调试信息

//     // 更新字段与调试：使用 dbg! 宏调试表达式值
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // 打印并赋值 60
//         height: 50
//     };
//     dbg!(&rect1);  // 调试引用，避免移动所有权

//     // 借用处理（函数借用结构体以避免所有权转移）
//     let rect2 = Rectangle{width: 30, height: 50};
//     println!("面积： {} 平方像素", area(&rect2));

//     // 调用结构体绑定的方法
//     println!("面积： {} 平方像素", rect2.area());

//     println!("can_hold: {}", rect2.can_hold(&rect1));

//     let sq = Rectangle::square(90);

//     println!("square: {:?}", sq);
    
//     println!("square.height: {}", sq.height);
    
// }




// ---------------- enum枚举, 注释, ----------------  
// 基础枚举
// enum IpAddrKind {
//     V4,
//     V6,
// }

// // 变体携带数据
// enum IpAddr {
//     V4(u8, u8, u8, u8), // 元组
//     V6(String) // 单个字段
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},  
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// // impl 枚举
// impl Message{
//     fn call(&self){
        
//     }
// }

// // OPtion枚举，处理空值
// // enum Option<T> {
// //     None,
// //     Some(T) // T: 泛型可以是任意类型
// // }


// // 模式匹配
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin{
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     // let four = IpAddrKind::V4;
//     // let six = IpAddrKind::V6;
//     // let home = IpAddr::V4(127, 0, 0, 1);
//     // let loopback = IpAddr::V6(String::from("::9527"));

//     let m = Message::Write(String::from("Jamie!"));
//     m.call();


//     // let some_number = Some(5);
//     // let some_char = Some('j');
//     // let absent_number: Option<i32> = None;  //指定类型以避免推断错误

// }



// // ----------------  循环语句  ----------------
// fn main() {
//     // loop {
//     //     println!("无限循环！");
//     //     break; // 退出循环
//     // }

//     let mut counter = 0;
//     loop {
//         counter += 1;
//         if counter == 10 {
//             break;
//         }
//     }
//     println!("计数器达到: {counter}");

//     counter = 0;
//     // 循环返回值
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 3;  // break 可以携带值返回给result
//         }
//     };
//     println!("结果: {}", result);

// }



// ---------------- 打印方法 ----------------
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     print!("Hello, ");
//     println!("world!");  // 输出: Hello, world!（换行）
    
//     let x = 42;
//     println!("x 的值为: {}", x);  // 输出: x 的值为: 42
    
//     println!("{} + {} = {}", 1, 2, 1 + 2);  // 输出: 1 + 2 = 3


//     // 格式化输出
//     /*
//         {:?}：调试格式（用于 Debug trait）。
//         {:#?}：美化调试格式（多行缩进）。
//         {:b}：二进制，{:x}：十六进制，{:o}：八进制。
//         {:.2}：浮点数精度（小数点后两位）。 示例：
//     */ 
//     let pi = 3.141592;
//     println!("Pi 约为 {:.2}", pi);  // 输出: Pi 约为 3.14
    
//     println!("二进制: {:b}", 10);  // 输出: 二进制: 1010

//     // 参数命名打印
//     println!("{subject} {verb} {object}",
//              object="懒狗",
//              subject="快速的棕色狐狸",
//              verb="跳过");  // 输出: 快速的棕色狐狸 跳过 懒狗
             

//     // debug宏: dbg! , 用于打印表达式以及源代码位置
//     let x = 5;
//     let y = dbg!(x * 2);  // 打印: [src/main.rs:3:13] x * 2 = 10，返回 10
//     dbg!(y + 1);  // 打印: [src/main.rs:4:5] y + 1 = 11


//     // Debug trait：结构化调试打印
//     let rect = Rectangle { width: 30, height: 50 };
//     println!("rect 是 {:?}", rect);  // 输出: rect 是 Rectangle { width: 30, height: 50 }
//     println!("rect 是 {:#?}", rect);  // 美化输出，多行缩进

// }


// ---------------- 模块 ----------------

// 模块定义
mod garden {
    // 由于模块中的函数默认是私有的 private，所以需要使用pub让外部调用
    pub fn plant(){
        println!("种植蔬菜!");
    }
}

fn main(){
    garden::plant(); // 在同一文件中访问模块中的函数
}