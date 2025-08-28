//any variable and functions should be named in snake case(small letters and _ )
//pubkey: 6T68GqyFLyB7yRZfWok5XU8dhki9hV8vtqY9Sr9nar3h
fn main() {
    print_hello();
    print_height(200);

    //by default all variable in rust are immutable
    // we can make them mutable by writing mut
    let mut _x = {
        let price = 10;
        let qty = 5;
        price * qty
    };

    println!("Result is : {} ", _x);

    println!("the add function returns {}", add(10, 10));

    println!("Your BMI is {:.2} ", bmi(1.5, 70.0)); //:.2 makes sure that it only prints the result upto 2 decimal places

    // when using const to declare variable, it is necessary to write the variable name in uppper case and assign the type literal
    //we can also use const to declare glebal variable(outisde main function)
    const AYU: i32 = 20;
    println!("\n\n THE Avlaue of AYU is {} ", AYU);
}

fn print_hello() {
    println!("Hello");
}

fn print_height(height: u32) {
    println!("My height is {} cm ", height)
}

// Expressions and Statements
// Expression: anything that returns a value, example= 5,true,false, add(3,4), if else conditions
// EXPRESSIONS DOESN'T END WITH SEMICOLON
//statement: anything that does not return a value
//examples: variable declarations( let x= 5), function definitions
// ALMOST ALL STATEMENTS IN RUST END WITH SEMICOLONS

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bmi(_height: f64, _weight: f64) -> f64 {
    _weight / (_height * _height)
}
