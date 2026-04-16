trait Description {
    fn describe();
}

struct Car;
struct Book;
struct Person;

impl Description for Car {
    fn describe() {
        println!("I am a car");
    }
}

impl Description for Book {
    fn describe() {
        println!("I am a book");
    }
}

impl Description for Person {
    fn describe() {
        println!("I am a Person");
    }
}

fn main() {
    Car::describe();
    Book::describe();
    Person::describe();
}
