fn object () {
    // 元组长度固定, 一旦声明, 长度不会增大或者缩小
    // 元组可以通过解构得到值
    // 也可以当做js对象通过索引获得值
    let tup: (i64, i64, &str) = (34, 45, "67");
    let (x, y, z) = tup;
    println!("{}", tup.0)
}