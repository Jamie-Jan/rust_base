// -------------------------- 所有权 --------------------------

// 1、所有权介绍
/**
 * what: 每个值都有一个 所有者 owner，负责在值超出作用域时释放它。
 * why: rust用所有权管堆内存，而不依赖垃圾回收器;
 * 
 * three rules:
 *      1) 每个值都有一个所有者变量;
 *      2）值在任一时刻只有一个所有者;
 *      3）当所有者超出作用域时，值会被丢弃（drop）;
 * 
 * 栈 VS 堆:
 *      栈数据（i32）固定大小，堆数据（String）动态大小; 所有权要管理堆数据;
 * 
 * why is it important:
 *      防止双重释放，使用后释放和数据竞争;
 * 
 * **/
 

// fn main() {
//     let s = String::from("jamie!");  // s 是 "jamie!" 的所有者
//     // 此处可以用s所有者变量
//     println!("{s}");
// } // 只要超出了作用域，String会被drop，内存释放;

/*
    原理: String是堆分配的，当main运行结束时，Rust会自动调用drop方法释放内存。
    固定大小的类型，i32则在栈上，不涉及所有权复杂性;
*/ 


// 2、移动 Move
// what: 将所有权变量，赋值给另一个变量时，则会将所有权 ”转移“;
// why: 可以防止多个所有者;

// fn takes_ownership(s: String) { // 接收参数时获得所有权;
//     println!("s: {s}");
// }

// fn main() {
//     let s = String::from("jamie!"); 
//     takes_ownership(s);
//     // error: 所有权已经转移给takes_ownership的s参数
//     // println!("{s}");
// } 


// 3、复制 Copy
/*
    Why: 某些类实现 Copy trait, 不会移动而是复制（
        i32,bool,f64,char,以及只含Copy类型的元组
    )。
*/ 

// fn main(){
//     let x: i32 = 5;  // 栈数据，需要copy
//     let y = x;  // 复制 x 的值
//     println!("x: {x}, y: {y}");  // 两者都有效

//     let s1 = String::from("jamie");  // 堆数据，非copy
//     // let s2 = s1;  // 所有权移动
//     // value borrowed here after move
//     // println!("s1: {s1}");  // 所有权被转移，报错

//     /*
//     copy类型的复制只能支持栈的固定值数据，无法copy动态数据，占用内存且不安全
//     建议使用#[derive(Copy, Clone)]为自定义类型添加（仅限栈数据）
//     */ 
//     // Clone：显示复制，非Copy类型可以用clone()实现深拷贝;
//     let s2 = s1.clone();  // 只是复制了一个所有权给另一个变量
//     println!("s1: {s1}");  // 所有权还在
// }



// 4、借用（Borrowing）
/*

借用允许临时访问值，而不是转移所有权，使用引用 & 。
    - 任何时候，只能有一个可变借用（&mut）,或多个不可变借用(&)，两者不能同时;
    - 引用必须有效;（必须要使用）
*/ 

// // 1) 不可变借用 &String
// fn calculate_length(s: &String) -> usize{  
//     s.len() // 不修改 s
// } // 借用结束

// // 2) 可变借用 &mut String
// fn change(s: &mut String) {
//     s.push_str(" sui");
// }

// fn main(){

//     // 不可变
//     // let s = String::from("solana");
//     // let len = calculate_length(&s);  // 传递引用
//     // println!("长度：{}, 值: {}", len, s); // s仍然有效

//     // 可变
//     let mut s = String::from("solana");
//     change(&mut s);
//     println!("s: {s}");


//     // 注意：不允许同时可变与不可变借用
//     let r1 = &s;  // 不可变借用
//     let r2 = &s;  // 不可变借用
//     // 报错前提是，下面必须要引用r1或者r2不可变借用的变量，
//     // 否则编译器会自动优化，判断为并未有不可变借用
//     // mutable borrow occurs here
//     // let r3 = &mut s;  // 可变借用，报错
//     println!("r1: {r1}");

// }



// 5、切片 Slices
// 切片是借用的一部分数据，字符串切片 &str;

// fn first_word(s: &str) -> &str { //接收 &str String的借用
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return &s[0..i]
//         }
//     }
//     &s[..]
// }
// fn main(){
//     let s = String::from("solana sui!");
//     let word = first_word(&s);
//     println!("第一个单词: {word}");
// }
/*
总结:
    &str 是字符串的借用视图。切片如 &s[2..5]。防止修改底层数据以避免失效引用;
*/


// 6、所有权与函数返回：函数可以返回所有权
// fn gives_ownership() -> String{
//     String::from("solana") // 返回新值，所有权转移;
// }

// fn takes_and_gives_back(s: String) -> String{
//     s // 接收所有权并返回;
// }

// fn main(){
//     let s1 = gives_ownership();
//     let s2 = String::from("sui");
//     let s3 = takes_and_gives_back(s2); // s2移动，s3获得返回的所有权
// }


// 7、Drop 与 RALL
/*
Drop trait：
    类型超出作用域时自动调用 drop 方法。

RAII（Resource Acquisition Is Initialization）：
    资源在创建时获取，销毁时释放。
*/ 
struct Custom{
    data: String,
}

impl Drop for Custom{
    fn drop(&mut self) {
        println!("Dropping: {}", self.data);
    }
}

fn main(){
    let c = Custom{data: String::from("solana!")};
    println!("waiting...");
    
} // drop会自动执行，并输入dropping的内容


// -------------------------- 总结 -------------------------- 
/*
    - 避免不必要的 clone：优先借用，clone 只在必要时。
    - mut 的使用：只在需要修改时用 mut。
    - 作用域控制：用 {} 显式限制作用域，早释放资源。
    - 常见错误：
        - 使用已移动值：编译错误，确保不重复使用。
        - 同时借用冲突：如在循环中借用 vector 同时 push（用索引代替）。
        - 悬垂引用：Rust 编译器防止，如返回局部变量的引用（错误）。
    - 与生命周期结合：借用涉及生命周期（'a），详见生命周期教程。
    - 性能：所有权是零成本抽象，编译时检查。

*/ 