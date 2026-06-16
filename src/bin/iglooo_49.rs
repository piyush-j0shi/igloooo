trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    r: f64,
}
struct Rectangle {
    l: f64,
    b: f64,
}

impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
    }
    fn area(&self) -> f64 {
        &self.r * &self.r
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        &self.l * &self.b
    }
}

fn print_shpae(shape: &dyn Shape) {
    println!("{}", shape.name());
    println!("{}", shape.area());
}

fn main() {
    let circle = Circle { r: 3.0 };
    let rectangle = Rectangle { l: 3.0, b: 3.0 };

    print_shpae(&rectangle);
    print_shpae(&circle);
}
