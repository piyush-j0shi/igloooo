#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn sawp(self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }
}

fn main() {
    let befores_swapped = Pair { a: 5, b: 6 };
    println!("Before swapped : {:?}", befores_swapped);

    let after_swapped = befores_swapped.sawp();
    println!("After swapped : {:?}", after_swapped);
}
