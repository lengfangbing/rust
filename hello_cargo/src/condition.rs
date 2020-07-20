fn main () {
  let mut x = 5;
  // 条件声明
  if x == 5 {
    println!("x is {}", x);
    x = x+1;
  } else {
    println!("x is not {}", x);
  }
  if x != 5 {
    println!("x is {} now", x);
  }
  let x = if true {
    5
  } else {
    6
  };
}