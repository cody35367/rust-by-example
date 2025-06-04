use std::convert::From;
// use std::convert::Into;

#[derive(Debug)]
struct Number {
    _value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { _value: item }
    }
}

// Not needed, from gives you into but into does not give from
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { _value: self }
//     }
// }

fn main() {
    let my_str = "hello";
    let _my_string = String::from(my_str);

    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}