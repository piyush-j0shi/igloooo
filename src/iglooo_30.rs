trait Greet {
    fn greet(&self);
    fn greet_louder(&self) {
        self.greet();
        println!("!!!");
    }
}

struct English {
    name: String,
}

struct Spanish {
    name: String,
}

impl Greet for English {
    fn greet(&self) {
        println!("Hello {:?}", self.name);
    }
}

impl Greet for Spanish {
    fn greet(&self) {
        println!("Hola {:?}", self.name);
    }
}

fn main() {
    let english = English {
        name: String::from("user"),
    };

    let spanish = Spanish {
        name: String::from("user"),
    };

    english.greet();
    english.greet_louder();

    spanish.greet();
    spanish.greet_louder();
}
