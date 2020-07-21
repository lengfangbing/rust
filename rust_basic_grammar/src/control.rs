fn condition() {
    let x = 32;
    if x == 35 {
        println!("{} equals 32", x);
    }
    let y = if x == 32 {
        x + 1
    } else {
        x
    };
    println!("y is {} ", y)
}

fn loop_fn() {
    let mut x = 32;
    loop {
        if x >= 35 {
            println!("x is {} now", x)
        }
        x = x + 1
    }
}

fn loop_break() {
    let mut x = 0;
    let res = loop {
        x += 1;
        if x == 10 {
            break x;
        }
    };
    println!("x is {} now", res)
}

fn while_fn() {
    let mut flag = 0;
    while flag <= 5 {
        flag += 1;
    }
    println!("flag is {} now", flag);
}

pub fn for_fn() {
    // for loop 更为安全, 因为while需要手动维护index(flag), 如果出现数组索引越界, 代码将会panic
    let mut res = String::from("str: ");
    let arr: [&str; 5] = ["10", "20", "30", "40", "50"];
    for element in arr.iter() {
        res += element;
    }
    println!("the string plus result is \n{}", res)
}

fn for_demo () {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
