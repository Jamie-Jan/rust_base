// ----------------------- 引用与借用 ------------------------
/* 
    what: 引用与借用是所有权系统的扩展部分。
        - 引用 &T: 一个指向类型T的值的指针，不拥有值;
        - 借用: 创建引用的过程。借用是临时的，在作用域结束时失效;
        

    why: 作用是允许访问数据而不是所有权转移，保证内存安全问题，避免不必要copy

    how: 
        借用: 在编译时强制执行;
        引用: 通过 & 表示，指向值的指针;

    类型: 
        - 不可变引用: &T -> 读访问，不能修改值;
        - 可变引用: &mut T -> 读写访问，可以修改值;

    解引用: 用 * 访问引用的值，如: *val
    
*/

// 1）不可变借用: 不可变借用允许多个同时存在，因为他们不修改数据;
// 总结: 函数借用s, 不转移所有权。多个 & 借用，只读;
// Rust中允许无限个不可变借用;
fn print_length(s: &String) {
    println!("长度: {}", s.len());
}


// 2）可变借用
fn append_world(s: &mut String){
    // 此时已经对借用的可变值追加了字符串
    s.push_str(", how do you do?");
}


// 3）借用规则
/* 
    1.任何只，在给定作用域内，可以有:
        1）一个可变引用;
            或
        2）任意个不可变引用，但不能同时拥有两者;
    
    2.引用必须始终有效（无悬垂引用）
    
    3.借用不能超过所有者的生命周期
*/

// 4）悬垂引用： 防止返回局部变量的引用，因为所有者超出作用域会导致引用悬垂;
// 编译报错: expected named lifetime parameter
/* 
fn dangle() -> &String {
    let s = String::from("sui"); // 局部变量
    &s // 返回局部变量
} // s drop, 引用无效
*/
// 正确示例: 静态引用
fn static_ref() -> &'static str {
    // 'static 生命周期，在编译后，整个程序运行期间都有效;
    "hello!"  // 字符串字面量是 'static 
}

// 5）引用与切片: 切片是引用的子集，数组或字符串的部分;
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // 检查当前是否有空格字符串，有则将第一个单词返回
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

// 6）引用与所有权交互
/* 
    - 借用后不能移动: 借用存在时，所有这不能被移动;
    - 解引用强制: 某些操作需要 * ，但方法调用隐式解引用;
    - Deref trait: 自定义类型可以实现Deref以像引用一样行为;
*/

fn main() {

    let x = 5;
    let y = &x; // y是x的不可变引用
    println!("x: {x}, y: {y}");
    
    // 总结: y借用x，但不拥有。x仍有效，引用的是栈上的指针，指向x的位置;
    // `y` is a `&` reference, so the data it refers to cannot be writte
    // *y = 10;  // 错误，不可变引用不能修改值
    // println!("x: {x}, y: {y}");

    // 不可变借用
    let s = String::from("Jamie");
    print_length(&s);  // 传递引用
    print_length(&s);  // 多次调用
    println!("原值: {s}");

    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}, r2: {r2}");

    let mut s2 = String::from("Jamie");
    println!("s2原值: {s2}");
    append_world(&mut s2);
    println!("s2修改后值: {s2}");

    // 注意：借用期间，s3不允许被其他方式访问!
    let mut s3 = String::from("Jamie");
    let r3 = &mut s3;  // 可变被借用了

    // 报错：必须要在第一个被借用的变量被使用的时候才会报错!
    // second mutable borrow occurs here
    // println!("s3: {s3}");  // 不允许再被其他方式访问
    // second mutable borrow occurs here
    // let r2 = &mut s3;
    // println!("{}, {}", s3, r3);

    // 借用冲突
    let mut s4 = String::from("sonic");
    let r4 = &s4;  // 不可变
    let r5 = &s4;  

    // let r6 = &mut s4;  // 报错： mutable borrow occurs here
    println!("{}, {}", r4, r5);

    // 引用与切片
    let s5 = String::from("sonic bnb");
    let word = first_word(&s5);  // 借用
    println!("word: {word}");

    // cannot borrow as mutable
    // 切片借用规则相同，修改s5会使word失效，但借用checker防止它;
    // s5.clear();  // 报错: word 借用期间不能修改s5

    // Deref强制
    let s6 = String::from("sui eth!");
    let r6 = &s6;
    println!("{}", r6.len()); // 隐式解引用: *r6.len()
    
}


// ----------------------- 总结：----------------------- 
/* 
    - 优先借用: 避免 clone，除非必要;
    - 最小借用作用域: 用 {} 限制借用，早释放锁;
    - mut 只在必要时: 减少可变借用以允许更多不可变访问;

    - 常见问题:
        - 借用冲突: 在循环中借用集合时修改（用索引或迭代器替代）
        - 悬垂引用：返回函数局部引用（用所有权返回或生命周期）
        - 未mut变量: 借用&mut时，所有者必须mut

    - 与生命周期结合: 复杂借用需生命周期注解（如 'a）

    - 性能: 引用是零成本，编译时检查无运行时开销;
*/



