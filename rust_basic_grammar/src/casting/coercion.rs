// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

fn coercion() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let decimal = 653242.45f32;
    // 隐式类型转换会不断加上或减去直到在转换的类型范围内
    let integer: u8 = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!("{}", integer);
    // // 1000 - 256 - 256 - 256 = 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-100i8) as u8);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);
}

pub fn start() {
    coercion();
}