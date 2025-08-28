fn main() {
    let a: Myenum = Myenum::A;
    let b: Myenum = Myenum::B(5);
    let c: Myenum = Myenum::C { x: 10, y: 20 };

    println!(" {:?} ", a);
    println!(" {:?} ", b);
    println!(" {:?} ", c);

    if let Myenum::B(val) = b {
        println!("value of B matches: {} ", val);
    }
    if let Myenum::C { x, y } = c {
        println!("value of c matches: {} {} ", x, y);
    }
}

#[derive(Debug)]
enum Myenum {
    A,
    B(i32),
    C { x: u32, y: u16 },
}
