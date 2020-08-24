use std::fmt::{Display, Debug};

struct Point<U> {
    x: U,
    y: (U, String),
}

enum Test<T> {
    Data(Option<T>)
}

impl<T> Point<T> {
    fn complex(&self) -> (&T, &T, &String) {
        (&self.x, &self.y.0, &self.y.1)
    }
}

pub fn generic() {
    let point = Point {
        x: 2.5,
        y: (22.0, "lfb".to_owned()),
    };
    let test = Test::Data(Some(5));
    println!("{:#?}", point.complex());
}
pub trait Summary {
    // 定义签名
    // fn summarize(&self) -> String;
    // 定义默认实现, 可以通过impl重载
    fn summarize(&self) -> String {
        "default summarize result".to_owned()
    }
}
struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", &self.title, &self.author, &self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}
// 寻找vec中最大的元素
fn largest<T>(list: &[T]) -> &T
    where T: PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

pub fn trait_test() {
    let number_list = vec![13, 45, 23, 654, 12];
    let result = largest(&number_list);
    println!("number_list largest value is {}", result);

    let char_list = vec!['l', 'f', 'b', 'w', 'm'];
    let result = largest(&char_list);
    println!("char_list largest value is {}", result);
}
// + 可以指定多个trait bound
fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}
// where定义更清晰的参数类型
fn some_function<T, U>(t: T, u: U) -> i64
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}