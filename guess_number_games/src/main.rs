// 猜字符游戏
/*
    1.用rand模块生成一个 1-100的随机数字
    2.通过loop循环让用户输入猜测的数字
    3.通过match判断用户输入的数字是否正确
        - 对用户输入的数字进行判断是否是数字
        - 去除用户输入的空格内容
        - 使用cmp比较数字
        - 使用Ordering模块输出数字大小
*/
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // stdin获取用户输入，read_line读取用户输入的内容，expect判断异常并输出
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // match后跟表达式
        // 对guess左右两边空格去除, 并通过parse将字符串转为u32类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp(): 比较方法，返回 Ordering 枚举类型
        // 比较 guess 和 secret_number 的大小关系
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }


    }
}
