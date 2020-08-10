fn array_demo() {
    // 数组长度是固定长度的, 而且数组的每个元素的类型必须相同
    // 可变数组建议使用vector
    let arr = [1, 2, 3, 4];
    // 数组声明类型, 第一个表示类型, 第二个表示长度
    let arr_with_type: [i64; 4] = [1, 2, 3, -4];
    // 可以这样声明每一个元素都相同的数组
    let same_val_arr = [3; 5];
    println!("{}", arr.len())
}