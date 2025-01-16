fn main() {
    // Primitive Data types
    // int, float, bool, char

    //Integer
    //Rust has signed(+ and -) and unsigned interger(only +) types of different sizes
    //Signed Integers: i8, i16, i32, i64, i128
    //unsigned Integers: u8, u16, u32, u64, u128.
    println!("Primitive Data types:-\n");

    println!("1.INTEGERS ");

    let x: i32 = 42;
    let y: i32 = -42;
    let z: u64 = 100;

    println!("Postive Signed Integer: {}", x);
    println!("Negative Signed Integer: {}", y);
    println!("Unsigned Integer: {}", z);

    //diff between i32(32 bits) and i64(64 bits)
    //range:
    // i32= 2^31= -2147483648 to +2147483648;
    // i64= 2^63= -9223372036854775808 to +9223372036854775808;
    let max32: i32 = 2147483647;
    let max64: i64 = 9223372036854775807;

    println!("Max value of i32: {}", max32);
    println!("Max value of i64: {}\n", max64);
    // Floats [Floating data types]
    //f32, f64

    println!("2.FLOAT ");

    let pi: f64 = 3.14;
    println!("Value of pi: {}\n", pi);

    //Bool
    println!("3.BOOL ");

    let is_honest: bool = true;
    let _is_fool: bool = false;

    println!("is honest {} \n", is_honest);

    //character type -char
    println!("4.CHAR");

    let letter: char = 'a';
    println!("the letter is {}", letter);

    println!("\n\n");

    //Compound Data types
    //arrays, tuples, slices, strings(slice string)

    println!("Compound Data types:-\n");

    //Arrays
    println!("1.Array");

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("the array is {:?}", numbers); //:? is used to print all the numbers in array
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("the first fruit is {}", fruits[0]);
    println!("the second fruit is {}", fruits[1]);
    println!("the third fruit is {}\n", fruits[2]);

    //Tuples
    print!("2.Tuple\n");

    let human: (&str, i32, bool) = ("Ayush", 20, false);
    println!("Human Tuple: {:?}", human);

    let mix_tuple = ("yadav", 40, true, [1, 2, 3, 4]);
    println!("Mix Tuple: {:?}\n", mix_tuple);

    //Slices: [1,2,3,4,5,]
    print!("3. Slices\n");
    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?} ", number_slice);

    let animal_slice: &[&str] = &["tuger", "elephant", "human"];
    println!("Animal Slice: {:?} ", animal_slice);

    let lang_slice: &[String] = &["HTML".to_string(), "CSS".to_string(), "JS".to_string()];
    println!("Lang Slice: {:?} ", lang_slice);

    //String Vs String Slices(&str)
    //Strings [growable, mutable, owned string type]

    let mut life_is: String = String::from("boring"); //by adding mut we make it mutable
    println!("Before Life is {} ", life_is);

    life_is.push_str(" and uselss.");
    println!("after Life is {} ", life_is);

    //&str(String slice)

    let string: String = String::from("Hello world");
    let slice: &str = &string;
    println!("The Slice value is: {}", slice);
}
