trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let circle = Circle { radius: 20.0 };
    let rectangle = Rectangle {
        width: 3.0,
        height: 2.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Circle perimeter: {}", circle.perimeter());

    println!("Rectangle area: {}", rectangle.area());
    println!("Rectangle perimeter: {}", rectangle.perimeter());
}
