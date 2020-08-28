use std::thread;
use std::time::Duration;

fn thread_spawn() {
    // 闭包仍然接收参数和标志
    thread::spawn(move || {
        for i in 1..10 {
            println!("{} in spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("{}", i);
    }
    // 主线程结束后, 新线程也会结束, 不会管新线程是否执行完毕
    // 这种方式创建的线程之间不会有执行顺序, Rust不能保证线程的运行顺序
}

fn join_thread() {
    let handle = thread::spawn(|| {
        // 前闭后开
        for i in 1..10 {
            println!("{} in spawn", i);
        }
    });
    // join会阻塞当前线程直到调用者的线程结束
    handle.join();
    for i in 1..5 {
        println!("{}", i);
    }
}

pub fn thread_test () {
    // thread_spawn();
    join_thread();
}