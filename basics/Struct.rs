fn main() {
    let murgi = String::from("murgi");
    let birddd = Birds {
        name: murgi,
        attack: 10,
    };
    birddd.printname();

    println!(
        "trait of anime is {} ,  {} ",
        birddd.can_fly(),
        birddd.is_animal()
    );
}

struct Birds {
    name: String,
    attack: u32,
}

//to implement the struct
impl Birds {
    fn printname(&self) {
        println!(
            " the name of the bird is {} and its attack is {} ",
            self.name, self.attack
        );
    }
}

impl Animal for Birds {
    fn can_fly(&self) -> bool {
        false
    }
    fn is_animal(&self) -> bool {
        true
    }
}

trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        false
    }
}
