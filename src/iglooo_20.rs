fn print_pair<T: std::fmt::Display>(a: T, b: T) {
    println!("a = {}, b = {}", a, b);
}

fn main() {
    print_pair(5, 6);
}
