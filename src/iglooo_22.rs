#[derive(Debug)]
struct MyBox<T> {
    some_value: T,
}

impl<T> MyBox<T> {
    fn swap(&self) -> &T {
        &self.some_value
    }

    fn into_ineer(self) -> T {
        self.some_value
    }

    fn map<U, F: Fn(T) -> U>(self, f: F) -> MyBox<U> {
        let some_other_value: U = f(self.some_value);
        MyBox {
            some_value: some_other_value,
        }
    }
}

fn main() {
    let some_struct = MyBox { some_value: 2 };
    let some_swap = some_struct.swap();
    let some_map = some_struct.map(|x| x + 1);
    let some_into_ineer = some_struct.into_ineer();
}
