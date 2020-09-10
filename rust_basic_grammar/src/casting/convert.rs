use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn try_from() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
// 如果你为你的类型实现了 From,那么同时你也就免费获得了 Into.
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number{
    fn from(val: i32) -> Self {
        Number{ value: val*2 }
    }
}

fn convert() {
    let val = Number::from(32);
    println!("{:?}", val);
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

pub fn start() {
    convert();
    try_from();
}