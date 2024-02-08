struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I'm {}.", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I'm {} years old.", self.age);
        self
    }
}

fn main() {
    let p = Person {
        name: String::from("Tetsuya"),
        age: 34,
    };

    p.say_name().say_age();
}
