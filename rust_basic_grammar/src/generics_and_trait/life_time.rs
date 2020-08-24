use std::fmt::Display;
// 当包含引用且返回引用时, 并且调用者生命周期不一致, 需要指明生命周期
// 生命周期也是一种泛型
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

pub fn test() {
    let str1 = "long string is long";
    let result;
    {
        // str2的生命周期不够长, 在这一块代码执行完后str2就被销毁了
        // 而后面我们使用了result
        let str2 = "xyz";
        result = longest(&str1, &str2);
    }
    println!("{}", result);
    let novel = "Call me Lfb. Some years ago...".to_owned();
    let first_sentence = novel.split('.')
        .next()
        .expect("No signal '.' ");
    let struct_life = StructLiftTime {
        name: first_sentence,
    };
    println!("{:#?}", struct_life);
}

#[derive(Debug)]
struct StructLiftTime<'a> {
    name: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}