// 使用模块的示例
use std::{cmp::Ordering, collections::HashMap};
use std::io::{self, Write};
// 不推荐使用, 因为这样Rust并不能推导出当前作用域代码所使用的库等
use std::fmt::*;

// 使用自己的模块示例
mod array;
mod control;
mod enums;
mod memory;
mod object;
mod reference;
mod string;
mod structs;
mod vector;

// 可以用use使用已经定义的mod模块
use array::create_array::create_array;
use structs::structs::structs_demo;
use vector::vector::vector_enum;

fn main() {
    println!("rust grammar");
    vector_enum();
}

