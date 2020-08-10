pub fn reference() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    // 下面的写法会报错, 因为相当于把变量(s1)所有权交给了方法(calculate_length)
    // 这样在方法内部使用完变量后就会drop()了, 这样会导致变量已经被销毁了, 后续无法访问
    // let len = calculate_length(s1);
    println!("{}", s1);
    // 使用可变引用就可以改变这个引用的值
    change(&mut s1);
    println!("{}", s1);
    reference_tips();
    // total: =>
    // 1. 任意时间段内, 要么只能有一个可变引用, 要么只能有多个不可变引用
    // 2. 引用必须总是有效的
}

fn calculate_length(str: &String) -> usize {
    // 借用的值是不可以修改的
    // str.push_str(", world");
    str.len()
}

// 这是可变引用, 所以数据可以发生改变
fn change(str: &mut String) {
    str.push_str(", world -- the ', world' from change function");
}

fn reference_tips() {
    // 同一个值只能同时存在!!一种!!引用
    // 下面的代码因为变量(s)同时存在不可变引用和可变引用而无法编译
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    // 因为可变变量(s)只能在当前作用域下存在一个可变引用
    // 所以下面注释的代码无法编译通过
    // 如果在使用r1, r2之后命名r4, r5则是可以的
    // 因为r1, r2之后不再使用了
    // let r4 = &mut s;
    // let r5 = &mut s;
    println!("{}, {} and {}", r1, r2, r3);
    // println!("{} and {}", r4, r5);
}

// dangle返回字符串的引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     // 返回字符串s的引用
//     &s
// }
// s离开作用域被丢弃, 内存被释放, 这个引用会指向无效的String

fn dangle_fix() -> String {
    let s = String::from("hello");
    s
}
// 字符串s的所有权被移动出去, 所以没有值被回收


// 有一种不同的引用 slice
pub fn slice() {
    let mut s = String::from("hello, world");
    // 使用[a..b]获取a->b之间的地址, a和b代表起止索引, 省略a表示从开头开始, 省略b表示从a到最后
    let h = &s[0..5];
    let word = first_word(h);
    println!("{}", h);
    s = String::new();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
