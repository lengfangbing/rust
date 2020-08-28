use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn simple_box() {
    let num = Box::new(5);
    println!("num is {}", num);
}

fn create_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn deref_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // &String本质是slice了一段String, 所以也是str
    hello(&MyBox::new("121"));
    hello(*MyBox::new("212"));
    // Rust简化了解引用强制多态
    // 使得传参时, Rust可以自动通过deref将Box<T>转为T
    // 前提是T必须实现了Deref trait
    // 在这里就是返回了字符串slice
}

// &str == &String
fn hello(name: &str) {
    println!("{}", name);
}

// 栈 : []
// 堆 : []
// Box -> &栈 -> &堆
// so: *Box == 栈 -> &堆
pub fn test_box() {
    // Box是一个栈上的引用, 而栈上的这个引用指向堆上的数据的引用, 这是Box数据结构的特点
    // Box是一种常用的智能指针
    // 而要使用Box上的值, 需要解引用, 因为我们拿的是堆上的数据的引用的引用, 所以我们需要解引用
    // 去获取堆上的数据的引用
    // 通过Deref trait将智能指针改为常规引用, 也就是*解引用
    deref_box();

    // Rust有三种情况会进行解引用强制多态
    // 1. T: Deref<Target=U>时: &T -> &U
    // 2. T: DerefMut<Target=U>时: &mut T -> &mut U
    // 3. T: Deref<Target=U>时: &mut T -> &U
    // 如果有一个&T, T实现了U的Deref, 咋可以直接到&U
}
