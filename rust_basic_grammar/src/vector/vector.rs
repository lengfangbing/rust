#[derive(Debug)]
enum Reject {
    RejectReason(u8),
    ShopBanned(bool),
    Performer(String),
}

pub fn vector_test () {
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    for i in &mut vec1 {
        *i *= 2;
        println!("{}", i);
    }
    println!("{}", vec1.get(3).unwrap());
}

pub fn vector_enum () {
    let vec: Vec<Reject> = vec![
        Reject::RejectReason(1),
        Reject::ShopBanned(true),
        Reject::Performer("lengfangbing".to_owned()),
    ];
    let value1 = vec.get(0);
    if value1.is_some() {
        println!("{:#?}", value1.unwrap());
    }
    let value2 = vec.get(1);
    if value2.is_some() {
        println!("{:#?}", value2.unwrap());
    }
}