struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("I'm {}.", self.name);
    }

    fn say_age(&self) {
        println!("I'm {} years old.", self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("Tarou"),
        age: 20,
    };

    p.say_name();
    p.say_age();
}
