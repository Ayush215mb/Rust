fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);

    // Shadowing the variable x with a new value
    let x = x + 1;
    println!("The value of x after shadowing is: {}", x);

    {
        // Shadowing the variable x within a new scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // The value of x outside the inner scope remains unchanged
    println!("The value of x outside the inner scope is: {}", x);
}
