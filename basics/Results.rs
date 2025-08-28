fn main() {
    let divide = divide(4, 2);

    // let res = divide.expect("We crashed");

    match divide {
        Ok(v) => println!(" option1: {} ", v),
        Err(v) => println!(" option1:  {:?} ", v),
    }

    // if divide.is_ok() {
    //     println!(" option2:  {} ", divide.unwrap());
    // }

    // println!(" option3: {} ", divide.unwrap());
    // println!(" option4: {} ", divide.unwrap_or(100)); // will print 100 if there is an error

    // println!("  option5: {} ", res); // will print a particular string if there is an error
}

#[derive(Debug)]
enum Myerror {
    Error1,
}

//Err, an enum that contains an error code
// Ok(value), A wrapper that contains a value

fn divide(dividend: i32, divisor: i32) -> Result<i32, Myerror> {
    if dividend % divisor != 0 {
        Err(Myerror::Error1)
    } else {
        Ok(dividend / divisor)
    }
}
