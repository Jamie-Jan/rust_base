// *************************** 高阶篇 ***************************


// ------------------- impl ------------------- 
// 用于 结构体 枚举 trait 定义方法和关联函数

// impl TypeName {
//     // 方法和关联函数
// }

/**
 * 
 * self: 表示实体引用
 *  -   &self(不可借用)
 *  -   &mut self(可变借用)
 *  -   self(所有权转移)
 * 
 * 关联函数: 不带self的函数，常用语构造函数，new
 * 
 * impl可多次定义在不同的模块中，Rust会合并它们;
 * 
 * 与 trait 的关系: impl可以实现 trait 接口，提供和多态;
 * 
 *
 **/

// ----------- 1、为结构体实现方法 -----------


// // 1）基本实现
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // 构造函数
    fn new(width: u32, height: u32) -> Self {
        // 实例化Rectangle对象并返回
        Rectangle {width, height}
    }

    // 计算面积方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可变借用方法： 缩放
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // 消耗 self 的方法
    fn into_square(self) -> Rectangle {
        // max() 判断width与height得到更大的值，然后实例化得到对象
        let side = self.width.max(self.height);
        Rectangle {width: side, height: side}
    }
}

// fn main(){
//     let mut rect1 = Rectangle::new(5, 10);
//     println!("面积: {}", rect1.area());

//     // 面积增加2倍
//     rect1.scale(2);
//     println!("新矩阵: {:?}", rect1);

//     let square = rect1.into_square();
//     println!("正方形: {:?}", square);

// }

// //  ---------------- 总结 ----------------
// /**
//  * fn new 是关联函数，通过 Type::function 调用。
//  * &self 方法借用实例，不修改它。
//  * &mut self 方法允许修改实例。
//  * self 方法消耗实例的所有权，适合转换操作。
//  * Self 是当前类型的别名，便于泛型中使用。
//  **/


// //  2）多个impl块
// // 作用: 一般用于大型项目中，按功能分组方法
// impl Rectangle {
//     fn area2(&self) -> u32 {

//     }
// }

// impl Rectangle {
//     fn scale2(&mut self, factor: u32) {
        
//     }
// }




// ----------- 2、为枚举实现方法 -----------
// #[derive(Debug)]
// enum Shape{
//     Circle(f64), // 半径
//     Square(f64), // 边长
// }

// impl Shape {
//     // 类似其他语言实现了抽象方法
//     // 方法使用 match 处理不同变体。枚举方法增强了类型的安全性和表达力。
//     fn area(&self) -> f64 {
//         match self {
//             // 对枚举的值绑定了对应的处理
//             Shape::Circle(r) => std::f64::consts::PI * r * r,
//             Shape::Square(s) => s * s,
//         }
//     }
// }

// fn main(){
//     let circle = Shape::Circle(5.0);
//     println!("圆面积: {}", circle.area());

//     let square = Shape::Square(6.0);
//     println!("正方形面积: {}", square.area());
// }



// ----------- 3、实现trait -----------
// trait 是 rust 的接口; impl trait for Type 为类型实现trait定义;
// trait Drawable { // 定义接口
//     fn draw(&self);
// }

// // 实现接口
// impl Drawable for Rectangle {
//     fn draw(&self) { //  实现接口draw
//         println!("绘制矩形: {} x {}", self.width, self.height);
//     }
// }

// fn main(){
//     let rect = Rectangle::new(5, 6);
//     rect.draw();  // 输出: 绘制矩形: 5 x 6
// }

// ----------- 总结 -----------
// trait 定义签名，impl 提供实现。类型可以实现多个 trait。

// 默认实现 和 trait bound
// trait可以有默认方法
// trait Summary {
//     fn summarize(&self) -> String {
//         String::from("「默认摘要」")
//     }
// }

// impl Summary for Rectangle {}  // 使用默认的实现

// // 默认方法允许可选实现。
// // T: Summary 是 trait bound，确保泛型参数实现了 trait。
// fn print_summary<T: Summary>(item: &T){
//     println!("{}", item.summarize());
// }

// fn main(){
//     let rect = Rectangle::new(5, 6);
//     print_summary(&rect);
// }



// ----------- 4、泛型 impl -----------
// impl可以是泛型的，支持trait bound
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// // 只为 Display 类型实现
// impl<T: std::fmt::Display> Point<T> {
//     fn display(&self) {
//         println!("「{}, {}」", self.x, self.y);
//     }
// }

// fn main(){
//     let p = Point{x: 5, y: 10};
//     println!("x: {}", p.x);
//     p.display();
// }
// ----------- 总结 -----------
// impl<T> 为所有 T 实现。
// bound 如 T: Display 限制实现范围，编译时检查。



// ----------- 5、高级主题: impl trait 和 dyn -----------
// 1）impl Trait: 作为返回类型，表示 ”某个实现Trait的类型“ 
// fn returns_drawable() -> impl Drawable{
//     Rectangle::new(1, 1)
// }
// // 2）dyn Trait: trait对象，用于运行时多态（有虚表开销）
// fn draw_it(d: &dyn Drawable) {
//     d.draw();
// }

// ----------- 总结 -----------
// 在需要异构集合时，如 Vec<Box<dyn Drawable>>



// ---------------------- 整体总结 ----------------------
/**
    - 方法 vs 关联函数：用 self 的叫方法，不用 self 的叫关联函数。
    - 私有性：用 pub 暴露方法/函数。
    - 避免过度 impl：保持类型内聚，方法应与类型数据相关。
    - trait 继承：trait 可以继承其他 trait，如 trait Super: Sub {}。
    - orphan rule：不能为外部类型实现外部 trait（防止冲突）。
    - 常见错误：
        - 借用规则违反：如在 &self 中修改字段（用 &mut self）。
        - 未实现 trait：编译错误，强制实现所有方法。
        - 泛型 bound 不足：添加 where 子句，如 impl<T> where T: Clone。
    - 性能：方法调用是静态分发的（零开销），除非用 dyn。
 * / 

