// 控制流
mod control;
// 数组
mod array;
// 元组
mod object;
// 字符串
mod string;
// 内存控制
mod memory;
// 引用
mod reference;
// 结构体
mod structs;

fn main() {
    println!("rust grammar");
    // control::for_fn();
    // reference::reference();
    // reference::slice();
    // structs::define_struct();
    let email = structs::get_comp_email();
    println!("{}", email);
}

