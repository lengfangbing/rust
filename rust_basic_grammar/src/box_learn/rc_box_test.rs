use List::{Cons, Nil};
use RcList::{RcCons, RcNil};
use RefCellList::{RefCellCons, RefCellNil};
use std::rc::Rc;
use std::cell::RefCell;

// Rc<T>只能用于单线程场景
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn danger_test() {
    // let a = Cons(7, Box::new(Cons(10, Box::new(Nil))));
    // // a的所有权被移动了
    // let b = Cons(5, Box::new(a));
    // let c = Cons(4, Box::new(a));
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

fn rc_test() {
    let a =  Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a is {}", Rc::strong_count(&a));
    let b = RcCons(1, Rc::clone(&a));
    println!("count after creating b is {}", Rc::strong_count(&a));
    {
        let c = RcCons(2, Rc::clone(&a));
        println!("count after creating c is {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope is {}", Rc::strong_count(&a));
    // Rc::clone只会增加引用计数, 而不是深拷贝, 所以不会花费很长时间
    // 所以Rc::clone不用考虑性能方面的问题
    // Rc::strong_count获取传入变量当前所有权所被拥有的个数
    //  Rc<T> 允许一个值有多个所有者，引用计数则确保只要任何所有者依然存在其值也保持有效。
}

#[derive(Debug)]
enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

fn ref_test() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCellCons(Rc::clone(&value), Rc::new(RefCellNil)));
    let b = RefCellCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RefCellCons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("a before is {:?}", a);
    println!("b before is {:?}", b);
    println!("c before is {:?}", c);
    *value.borrow_mut() += 10;
    println!("a after is {:?}", a);
    println!("b after is {:?}", b);
    println!("c after is {:?}", c);
}

// RefCell<T>是当你确定代码遵守了借用规则, 而编译器不能理解和确定的时候使用
// RefCell<T>也是用于单线程的
// Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
// 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。
pub fn test_rc() {
    // rc_test();
    ref_test();
}