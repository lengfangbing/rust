pub fn string () {
    // 这是字符串字面量
    // 字符串字面量不可变
    // 编译时就确定, 所以分配内存在栈上, 快速且高效
    let s: &str = "123";
    // String是分配到堆上, 所以可以存储编译时未知大小的内容
    // 内存分配到堆上就意味着比分配到栈上的字符串字面量要慢, 效率低
    let s: String = String::new();
    let mut s: String = String::from("initial value");
    s.push('a');
    s.push_str(",a string value");
    let s1 = "123".to_owned();
    let s2 = format!("{}{}", s1, s);
    // 使用format!可以拼接字符串, 并且不会使用字符串的所有权
    println!("s: {}, s1: {}, s2: {}", s, s1, s2 );
}

fn add_error () {
    let s = "456";
    let s1 = "123".to_owned();
    let s2 = s1 + &s;
    // error 因为上面的s1已经把所有权移动了, 所以s1此时不存在了
    // println!("{}", s1);
}

pub fn string_byte () {
    let s1 = "नमस्ते".to_owned();
    let s2 = "冷方冰";
    // len()返回的是字节数
    println!("{}", s1.len());
    println!("{}", s2.len());
    let s3 = &s1[0..3];
    // 下面的error是因为字符串截取是取的字节数,
    // 在[0-3)中所截取的并不一定能形成一个UTF-8的字符串, 所以panic错误了
    println!("{}", s3);
    let mut chars_result: String = "".to_owned();
    // 可以使用chars()去处理类型为UTF-8的字符串
    for char in s1.chars() {
        println!("{}", &char);
        chars_result.push_str(&char.to_string());
    }
    println!("{}", chars_result);
}