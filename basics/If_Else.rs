use std::io;

fn main() {
    let mut name = String::new();
    let mut roll_number = String::new();

    println!("Enter your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter your roll number:");
    io::stdin()
        .read_line(&mut roll_number)
        .expect("Failed to read line");

    let name = name.trim();
    let roll_number = roll_number.trim();

    println!("Name: {}", name);
    println!("Roll Number: {}", roll_number);
}
