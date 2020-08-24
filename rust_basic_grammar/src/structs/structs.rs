struct User {
    name: String,
    email: String,
    count: i64,
    active: bool,
}
impl User {
    fn email (&self) -> String {
        String::from(self.email.to_owned() + "@bytedance.com")
    }
}

pub fn get_comp_email () -> String{
    let user = User {
        name: "lengfangbing".to_owned(),
        email: "lengfangbing".to_owned(),
        count: 22,
        active: true,
    };
    user.email()
}

pub fn define_struct() {
    let mut user = User {
        email: String::from("lengfangbing@bytedance.com"),
        name: "fangbing".to_owned(),
        count: -0,
        active: true,
    };
    let mut name = user.name.clone();
    name = "123".to_owned();
    println!("{}", name);
    println!("{}", user.name);
    user.name = "123".to_owned();
    println!("{}", user.name);
    user = build_user(&"lfb".to_owned(), "fff".to_owned());
    println!("{}", user.email);
}

fn build_user(name: &String, email: String) -> User {
    User {
        name: String::from(name),
        email,
        count: 22,
        active: false,
    }
}

fn copy_user(u1: User) -> User {
    User {
        email: "lfb".to_owned(),
        name: "name".to_owned(),
        ..u1
    }
}

#[derive(Debug)]
struct Square {
    width: i64,
    height: i64,
}

pub fn structs_demo() {
    let sq = Square {
        width: 30,
        height: 50,
    };
    println!("sq is {:#?}", &sq);
    // 给calc_area这个方法借出去sq, 而不是给他所有权
    println!("{}", calc_area(&sq));
    println!("{}", sq.area(true))
}

// 这里只是借用square, 而不是需要所有权, 所以需要&
fn calc_area(square: &Square) -> i64 {
    square.width * square.height
}

// 为Square实现接口, 接口内定义方法等
// 每个结构体允许拥有多个接口impl
impl Square {
    fn area(&self, less_zero: bool) -> i64 {
        self.width * self.height * if less_zero {
            -1
        } else {
            1
        }
    }
}