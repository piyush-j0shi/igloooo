trait Speak {
    fn speak(&self) -> String;
}

struct Dog {}
impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

struct Cat {}
impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow".to_string()
    }
}

fn main() {
    let vector: Vec<Box<dyn Speak>> = vec![Box::new(Dog {}), Box::new(Cat {})];
    for speech in vector.iter() {
        println!("{}", speech.speak());
    }
}
