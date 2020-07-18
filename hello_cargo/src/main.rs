// const 定义常量 必须标注类型
const MAX_POINTS: u32 = 100_000;
const MIN_POINTS: i32 = -100_000;
fn main() {
    // mut 可变变量
    let mut x = "冷方冰";
    // 可以复用变量名
    // 本质是开启一个新变量, 也可以改变该变量的类型
    // 这个space其实就是一个崭新的变量
    // 如果使用mut 类型不一致就会报错
    let space = "    ";
    let space = space.len();
    let age: i64 = 21;
    let age: f64 = -20.1;
    let young: bool = true;
    let lastname: char = '😸';
    // 元组长度固定, 一旦声明, 长度不会增大或者缩小
    // 元组可以通过解构得到值
    // 也可以当做js对象通过索引获得值
    let tup: (i64, f64, &str) = (100, 1.5, "爱情");
    let (x, y, z) = tup;
    let x = tup.0;
    // 数组长度是固定长度的, 而且数组的每个元素的类型必须相同
    // 可变数组建议使用vector
    let arr = [1, 2, 3, 4];
    // 数组声明类型, 第一个表示类型, 第二个表示长度
    let arrWithType: [i64; 4] = [1, 2, 3, -4];
    // 可以这样声明每一个元素都相同的数组
    let sameValArr = [3; 5]; // 长度为5, 每个元素值都是3
}