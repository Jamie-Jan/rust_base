/* 
    泛型：generic type

    desc: 允许编写抽象、可重用的代码，而不是牺牲性能。
        - rust的泛型是零成本抽象: 在编译时单态化，生成具体类型的代码。
        - 确保类型安全，避免运行时浪费开销;
        - 泛型常用语函数、结构体、枚举和trait中;

    - what: 
        - 使用类型参数定义代码，允许在不同类型上重用; T 是占位符，在使用时替换为具体类型;
    - why:
        - 代码服用、类型安全、性能高;
    - how: 
        - 在函数、struct后用<参数>
    - 与trait的关系: 
        - 泛型常结合trait bound（如 T: Clone）限制类型
*/

// 1）基本定义
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         // 会报错: T未指定比较， 可能不支持;
//         // binary operation `>` cannot be applied to type `&T`
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 2) 泛型函数
// fn print_value<T>(value: T) {
//     // 报错: T需要实现Debug
//     println!("值: {:?}", value);
// }

// 3）泛型结构体和枚举
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T{
        &self.x
    }
}

enum Option<T> {
    Some(T),
    None,
}


// 4) Trait Bound
/* 
    what: 限制泛型类型必须实现某些trait。
    how: fn foo<T: Trait + Trait2>(arg: T)
    common bound: Copy/Clone/Debug/PartialEq/PartialOrd;
*/
use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("value: {:?}", value);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // 会报错: T未指定比较， 可能不支持;
        // binary operation `>` cannot be applied to type `&T`
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 5) Where 子句
// where 在签名后。适用于函数、impl、trait。
// fn some_function<T, U>(t: T, u: U) -> U
// where 
//     T: Debug + CLone,
//     U: Clone + PartialEq
// {
//     if t.clone() == u { // 错误！T 和 U 类型不同，不能比较

//     }
// }


// 6) 泛型 impl 和 trait

// impl: 
// bound 限制 impl 的范围
// impl<T: Debug> Point<T> { // 只为Debug类型实现;
//     fn debug_print(&self) {
//         println!("{:?}", self);
//     }
// }

// // trait: trait 本身可以泛型，但常见是方法内用泛型
// trait Summary<T> {
//     fn summarize(&self) -> T;
// }


// 7）关联类型与生命周期
trait Iterator {
    /* 
        泛型与生命周期：结合 'a，如 fn longest<'a, T>(x: &'a T, y: &'a T) -> &'a T。
        性能：单态化生成具体代码，可能增加二进制大小，但运行时零开销。
     */
    type Item;  // relation type
    fn next(&mut self) -> Option<Self::Item>;
}




fn main() {
    // let arr = [1, 2, 3];
    // largest(&arr);

    // print_value(5);       // T = i32
    // print_value("hello"); // T = &str

    let integer_point = Point {
        x: 5, y: 10
    };

    let float_point = Point {
        x: 1.0, y: 4.0
    };

    println!("整数点: {:?}", integer_point);
    println!("浮点: {:?}", float_point.x());
    // Point 为 T 生成具体类型。impl 为所有 T 实现方法。多参数如 <T, U> 允许不同类型，如 Point { x: 5, y: 3.14 }。

    let some_number = Option::Some(5);
    let absent: Option<i32> = Option::None;
    // 枚举变体持泛型值。标准库 Option 和 Result<T, E> 是泛型枚举。

    let numbers = vec![34, 50, 25, 100, 67];
    println!("最大值: {}", largest(&numbers));

}


/* 

    - 使用 bound 最小化：只添加必要 trait，避免过度限制。
    - 优先具体类型：泛型用于真正需要重用时。
    - ** turbofish 语法**：指定类型如 Vec::::new()，当推断失败时用。
    - 常见错误：
        - 未 bound：操作如 + 时错误（添加 T: Add）。
        - 类型不匹配：如混合 T 和 U 时，确保兼容。
        - 过度泛型：导致代码复杂，考虑 trait 对象（dyn Trait）用于运行时多态（有开销）。
        - 编译时间长：过多泛型展开，优化 bound 或用 Box。
    - 标准库示例：Vec、HashMap<K, V> – 研究它们的 impl。

*/
