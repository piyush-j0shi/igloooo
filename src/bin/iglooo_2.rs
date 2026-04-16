enum Shape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => 3.14 * radius * radius,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

fn main() {
    let area = Shape::Rectangle {
        width: 10.0,
        height: 10.0,
    };
    println!("area : {:?}", area.area());
}
