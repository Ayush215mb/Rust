use std::collections::HashMap;

//None to indicate failure or lack of value, and
// Some(Value), a tuple struct that wraps a value with type T.

fn divide(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    let divide1: Option<i32> = divide(4, 2);
    let divide2: Option<i32> = divide(2, 3);

    //unwrapping a `Some` variant will extract the value wrapped.
    println!(" {:?} unwraps to {} ", divide1, divide1.unwrap());

    //unwrapping a `None` variant will `panic!`
    println!(" {:?} unwraps to {} ", divide2, divide2.unwrap());
}
