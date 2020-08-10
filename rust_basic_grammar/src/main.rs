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
mod map;
mod demo;

use demo::black_pic_check::pic_check_main;

// 可以用use使用已经定义的mod模块
use array::create_array::create_array;
use structs::structs::structs_demo;
use vector::vector::vector_enum;

fn create_black_pic_map (shop_banned: u8, reject_reason: u8) -> HashMap<String, String> {
    let mut res = HashMap::new();
    res.insert("product_id".to_owned(), "2016207235".to_owned());
    res.insert("shop_banned".to_owned(), shop_banned.to_string());
    res.insert("reject_reason".to_owned(), reject_reason.to_string());
    res
}

fn main() {
    println!("rust grammar");
    let mut val = vec![];
    val.push(create_black_pic_map(1, 1));
    val.push(create_black_pic_map(1, 2));
    val.push(create_black_pic_map(1, 3));
    val.push(create_black_pic_map(2, 0));
    let res = pic_check_main::start_verify_many(&val);
    println!("{:#?}", res);
}

