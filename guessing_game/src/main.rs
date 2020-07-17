// rust 将少量的类型引入到每个程序作用域内, 需要
// 的类型不在的话需要use显式的将他引入到作用域内
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// main 函数是程序的入口点
fn main() {
    let stable = rand::thread_rng().gen_range(1, 101);
    // 方法加上!表示这不是一个函数, 而是一个宏
    println!("Guess the number");

    // 循环, break continue和其他的语法一样
    loop {
        println!("Please input your guess:");

        // 创建变量存储用户输入, 这里之后会介绍※ mut表示一个变量是可变的
        // ::语法表明new 是 String类型的一个关联函数. 关联函数
        // 是针对类型实现的, 这里表示new是根据String实现的, 而不是String的实例
        // 很像js中的static静态方法
        // new => constructor
        let mut guess = String::new();
        // &表示这个参数是一个引用, 这个在js也有, 可以理解不多说
        // &mut 后面会详细解释
        io::stdin().read_line(&mut guess)
          .expect("Failed to read line");

        // parse() 返回的Result类型有两种
        // Ok => 解析成功
        // Err => 解析错误
        let guess: u32 = match guess.trim().parse() {
          Ok(n) => n,
          Err(_) => continue,
        };
        // {} 占位符
        println!("You guessed: {}", guess);
        // 尝试写比较
        match guess.cmp(&stable) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
            println!("You win!");
            break;
          }
        }
    }
}
