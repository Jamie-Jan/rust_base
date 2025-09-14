// -------------------- Slices 切片 --------------------
/*
    what: slice是数据序列，指向连续内存块，不拥有数据，只借用;
    语法:
        - 不可变：&[T]
        - 可变: &mut [T]
    why: 零拷贝访问子集; 函数参数通用;
    与数组/向量的关系: 数组是固定大小，向量是动态; Slice可以从两者中创建;
    字符串slices: &str 是 &[u8] 的特殊形式，处理UTF-8;
*/

fn first_word(s: &str) -> &str {
    // 将&str转为&[u8]，切片才能通过索引获取字符，否则panic
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 求和函数
fn sum_slice(nums: &[i32]) -> i32 {
    let mut sum = 0;
    // &[i32] 接受数组或向量的借用。迭代用 for &item（解引用）。
    for &num in nums {
        sum += num;
    }
    sum
}

fn main() {
    // 1、创建 Slice
    // 1）常规创建
    let arr = [1, 2, 3, 4, 5]; // 数组
                               // [1..4] 切片顾头不顾尾;
    let slice = &arr[1..4]; // 创建slice
    println!("{:?}", slice);

    // 2）各种创建方式
    let vec = vec![10, 20, 30, 40, 50];
    let full = &vec[..]; // 全切片
    let first_three = &vec[0..3]; // [10, 20, 30]
    let last_two = &vec[3..]; // [40, 50]
    println!("{:?}", first_three);

    /*
        总结:Vec 和数组都支持。Slice 的 len() 返回元素数，
        get(i) 返回 Option<&T>（安全访问）。
    */

    // 3）字符串Slice（&str）
    let s = String::from("Sonic Sui");
    let first_word_name = first_word(&s);
    println!("{}", first_word_name);

    // cannot borrow as mutable
    // s.clear();  // 借用期间不能修改s

    // 4）函数中的Slice
    let arr4 = [1, 2, 3];
    let vec4 = vec![4, 5, 6];
    println!("{}", sum_slice(&arr4));
    println!("{}", sum_slice(&vec4));

    // 5) 多维Slice
    // 嵌套借用。复杂时考虑扁平化或专用 crate。
    let matrix = vec![vec![1, 2], vec![3, 4]];
    let row = &matrix[0][..]; // &[i32]: [1, 2]
    println!("{:?}", row);

    // 6) 高级：Unsafe 与 Split
    // Split: split_at() 分割 slice
    let arr6 = [1, 2, 3, 4, 5, 6, 7];
    // let (left, right) = arr6.split_at(2);  
    let (left, right) = arr6.split_at(2);  // 从索引2的地方开始切割
    println!("{:?}, {:?}", left, right);

    // Unsafe slice: 在unsafe块中，可以创建原始指针，但避免，除非必要;

    // Deref到slice: Vec 和 String 实现 Deref<Target=[T]>, 所以
    // &Vec可隐式转为&[T];
    
}


/* 
    总结:
        - 安全访问: 用get()而非 [index]， 避免panic;
        - 避免修改借用: 在借用时，不能修改底层集合;
        - UTF-8安全: 字符串slice时，用char_indices() 处理多字节字符;
        - 性能: Slice是零成本视图，无分配;
        
        - 常见问题:
            1) 索引越界: 用if let Some(v) = slice.get(i)
            2) 非字符边界切片: 如 &s[0..1]如果s是多字节（panic）;
            3) 借用冲突: 如借用slice同时push到vec（用临时变量或重组代码）

        - 生命周期: 复杂函数需生命周期注解（如: fn foo<'a>(s: &'a [T])）


*/

