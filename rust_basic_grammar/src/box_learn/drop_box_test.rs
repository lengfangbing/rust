#[derive(Debug)]
struct CustomSmartPointer <'a> {
    data: &'a str,
}

impl Drop for CustomSmartPointer<'_> {
    fn drop(&mut self) {
        println!("CustomSmartPointer with data: {} is dropping", self.data);
    }
}

fn impl_drop() {
    let a = CustomSmartPointer {
        data: "123",
    };
    let b = CustomSmartPointer {
        data: "456"
    };
    // 可以手动调用drop去释放一个变量, 不让Rust自动在变量作用域结束后就释放
    // drop是释放变量的方法, 传入引用没什么用...
    drop(a);
    println!("func finished!");
    // b先释放, 这更能很好的解释栈的数据结构特点
}

pub fn test_drop(){
    // Drop trait 是一个极其重要的trait, 他可以帮助我们清理计算机的内存等
    // 合理使用可以使程序安全平稳运行
    impl_drop();
}