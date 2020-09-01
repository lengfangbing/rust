// 从后端的角度去看待这些约束
// 这个相当于接口, 定义了实现该接口需要实现的方法, 也可以提供默认方法
// 实现该接口必须把所有的方法都实现
trait Draw {
    // 安全的trait不应包含返回值是self和泛型参数
    fn draw(&self, str_from_parent: String);
    fn get_str<'a>(&self, str: &'a str) -> &'a str {
        str
    }
}

// 相当于某个数据结构
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for components in self.components.iter() {
            components.draw("123".to_owned());
        }
    }
}

#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self, str: String) {
        println!("Button drawing...");
        println!("{:?}", self);
        println!("str from parent: {}", str);
        println!("Button drew");
    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self, str: String) {
        println!("SelectBox drawing...");
        println!("{:?}", self);
        println!("str from parent: {}", str);
        println!("SelectBox drew");
    }
}

fn start_button () {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 40,
                height: 15,
                label: "button".to_owned(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 15,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };

    screen.run();
}

pub fn start() {
    start_button();
}