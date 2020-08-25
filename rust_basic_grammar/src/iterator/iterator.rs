fn for_iter() {
    let vec = vec![1, 2, 3];
    // into_iter 获取所有权
    // iter_mut 获取可变引用
    // iter 获取不可变引用
    // let iter = vec.into_iter();
    // let iter = vec.iter_mut();
    let iter = vec.iter();
    for item in iter.as_slice() {
        println!("{}", item);
    }
    println!("{:?}", vec);
}

fn test_collect() {
    let v1: Vec<i32> = vec![0, 1, 2];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![1, 2, 3]);
}

#[derive(Debug)]
struct Shoe<'a> {
    size: u8,
    style: &'a str,
}

fn shoes_in_my_size(vec: Vec<Shoe>, size: u8) -> Vec<Shoe> {
    vec.into_iter()
        .filter(|x| x.size == size)
        .collect()
}

fn find_shoes() {
    let vec = vec![
        Shoe {
            size: 38,
            style: "旅游鞋",
        },
        Shoe {
            size: 43,
            style: "篮球鞋",
        },
        Shoe {
            size: 43,
            style: "限量款篮球鞋",
        },
        Shoe {
            size: 45,
            style: "足球鞋",
        }
    ];
    let my_size = 43 as u8;
    let my_shoes = shoes_in_my_size(vec, my_size);
    println!("{:?}", my_shoes);
}

pub fn test_iter() {
    for_iter();
    test_collect();
    find_shoes();
}