/*
    Rust 生命周期
        - What:
            - 生命周期表示值或引用的生存范围，从创建到销毁的过程;
        - Why:
            - 确保引用不指向已释放的内存，借用规则要求引用不能比所有者存活久;
        - How:
            - 'a (单引号+字幕)表示，如: &'aT。'a是泛型生命周期参数;

        - rules:
            - 每个引用都有生命周期
            - 函数签名中指定以帮助编译器;
            - 默认规则: 函数参数的生命周期独立，返回值的生命周期与参数相关;
        - 'elision rules': Rust自动省略简单情况的注解;
*/

// 1）无注解的简单借用
// 报错: 无注解时编译错误，因为返回的 &str 的生命周期不明。
// 编译器无法确定是 'x 还是 'y 的生命周期   
// fn longest(x: &str, y: &str) -> &str{
//     if x.len() > y.len() {
//         x 
//     } else {
//         y
//     }
// }


// 2) 显式生命周期注解: 在签名中添加 'a 指定关系;
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}


// 3）结构体中的生命周期: 结构体有引用时，必须注解生命周期;
#[derive(Debug)]
struct Excerpt<'a> {
    part: &'a str,
}


fn main() {
    let result2 = longest("sonic", "sui");
    println!("longest result2: {}", result2);

    let string1 = String::from("bnb");
    let result;
    {
        let string2 = String::from("eth cion");
        result = longest(&string1, &string2);
        println!("最长: {}", result);  // 有效: string2在销毁前使用
    }
    // 报错: result引用了string2, 此时string2已经被drop
    // borrowed value does not live long enough
    // println!("最长: {}", result);  

    let novel = String::

}
