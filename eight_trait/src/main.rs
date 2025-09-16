/*
    Trait:
        - what: 接口
        - why: 允许不同类型定义方法签名，实现多态和代码服用，不依赖集成;
*/

// 1) 定义一组trait
trait Summary {
    // 定义接口
    fn summarize(&self) -> String;

    // 默认方法
    fn read(&self) -> String {
        String::from("default read...")
    }
}

// 2) 实现trait接口
#[derive(Debug)]
struct Article {
    headline: String,
    content: String,
}

// 在结构体上实现接口的方法
impl Summary for Article {
    // impl Summary for Article 实现 trait。
    // 方法使用 self 访问字段。类型必须实现所有 trait 方法（除默认）。
    fn summarize(&self) -> String {
        format!("{}:  {}", self.headline, &self.content[0..])
    }
}

struct Book {
    title: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        String::from("default summarize...")
    }
} // 3) 使用默认的read

// 4) Trait Bound: 在泛型中使用trait作为约束bound
fn notify<T: Summary>(item: &T) {
    println!("notify: {}", item.summarize());
}

// 5) Trait 作为参数和返回类型
/*
    - 参数: 用impl Trait 或 &dyn Trait
    - 返回: impl Trait（静态分发）或 Box（动态分发）
*/

// impl Trait 表示“某个实现了 Trait 的类型”（不暴露具体类型）。用于抽象返回。
// fn returns_summarizer() -> impl Summary {
//     Article{}
// }

// 6) 关联类型
// Trait 可以定义关联类型，避免额外泛型参数;
trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 7) Trait 继承 和 Supertrait
// Trait可以依赖其他trait
trait Display: Summary {
    fn display(&self);
}

impl Display for Article {
    fn display(&self) {
        // 继承了Summary的summarize方法
        println!("show: {}", self.summarize());
    }
}

// 8）高级
/* 
    - 异步 trait：在 async fn 中使用，需要 nightly 或 async_trait crate。
    - 生命周期：Trait 方法可带 'a，如 fn foo<'a>(&'a self) -> &'a str。
    - Blanket impl：如 impl<T: Display> ToString for T {} – 为所有 Display 类型实现 ToString。

*/

fn main() {
    let article = Article {
        headline: String::from("Rust"),
        content: String::from("Rust 9527 666"),
    };
    println!("{}", article.summarize());

    let book = Book {
        title: String::from("book obj"),
    };
    println!("{}", book.read());

    let article2 = Article {
        headline: String::from("Sonic"),
        content: String::from("Sonic 9527 666"),
    };
    notify(&article2);

    // dyn Trait: 用于异构集合或运行时多态
    let summaries: Vec<Box<dyn Summary>> = vec![
        Box::new(Article {
            headline: String::from("sui"),
            content: String::from("sui 9527 666"),
        }),
        Box::new(Book {
            title: String::from("sui title"),
        }),
    ];
    for s in summaries {
        println!("{}", s.summarize());
    }

    let mut counter = Counter { count: 0 };
    println!("{:?}", counter.next());
}


/* 
    总结:
        - 设计 trait：保持小而专注（单一责任）。
        - 优先静态分发：用泛型和 impl Trait，避免 dyn 的开销。
        - Orphan rule：不能为外部 crate 的类型实现外部 trait（用 newtype 包装）。
        - 常见错误：
            - 未实现方法：编译错误，强制实现所有非默认方法。
            - Bound 不足：如调用未 bound 的方法（添加 T: Clone 等）。
            - 对象安全：dyn Trait 要求 trait 对象安全（无泛型方法、无 Self 返回等）。
            - 冲突实现：避免 diamond 继承问题（Rust 无类继承）。
        - 标准库 trait：如 Debug、Clone、PartialEq – 用 #[derive] 自动实现。
        - 性能：Trait 方法静态分发零开销；dyn 有虚调用开销。
*/
