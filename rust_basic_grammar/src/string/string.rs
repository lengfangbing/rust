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
}