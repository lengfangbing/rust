use std::io;
use std::fs::File;
use std::io::Read;

pub fn panic_read_to_string () -> Result<String, io::Error>{
    let mut s = "".to_owned();
    // ?表示如果存在错误就返回错误信息, 如果不存在错误就继续执行程序
    File::open("test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}