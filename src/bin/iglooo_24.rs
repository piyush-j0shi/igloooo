use std::fmt;

struct Wrapper<T> {
    k: T,
}

impl<T: std::fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({})", self.k)
    }
}

fn main() {
    let int_wrapper = Wrapper { k: 10 };
    println!("{}", int_wrapper);

    let string_wrapper = Wrapper { k: "hello" };
    println!("{}", string_wrapper);
}
