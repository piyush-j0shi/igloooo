struct Color {
    x: i32,
}
struct Point {
    y: i32,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.y)
    }
}

fn main() {
    let color = Color { x: 1 };
    let point = Point { y: 1 };

    println!("color : {}", color);
    println!("point : {}", point);
}
