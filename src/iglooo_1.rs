#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true;
        }

        false
    }

    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

// main function
fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("width : {:?}, height : {:?}", rect1.width, rect1.height);
    println!("area : {:?}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("can hold : {:?}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 30,
        height: 30,
    };

    println!("is square : {:?}", rect3.is_square());
}
