use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "hii");
    map.insert(1, "hii22");
    println!("{:?}", map);

    match map.get(&0) {
        Some(str1) => println!(" {} ", str1),
        None => println!("does not exist in map"),
    }

    match map.get(&2) {
        Some(str) => println!(" {} ", str),
        None => println!("does not exist in map"),
    }

    map.remove(&0);

    println!(" {:?} ", map);
}
