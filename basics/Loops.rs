fn main() {
    let mut x: u32 = 0;

    //for loops in rust is very similar to python
    for i in 0..10 {
        println!("{}", i);
    }

    //while
    while x < 10 {
        //switch statements
        match x {
            0 => println!("value is 0"),
            1 | 2 => println!("value is 1 or 2"),
            3..=10 => println!("the value is in between the range of 3-10"),
            _ => println!("Default"),
        }
        x += 1;

        if x == 7 {
            println!("X has reached its max value of {}", x);
            break;
        }
    }
}
