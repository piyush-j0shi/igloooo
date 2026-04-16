fn unwrap_or_default<T: Default>(opt: Option<T>) -> T {
    match opt {
        Some(value) => return value,
        None => {
            let some_value: T = Default::default();
            return some_value;
        }
    }
}

fn unwrap_or_else<T, F: Fn() -> T>(opt: Option<T>, f: F) -> T {
    match opt {
        Some(value) => return value,
        None => return f(),
    }
}

fn main() {
    let some_new_value = unwrap_or_default(None::<f32>);
    println!("Some new value : {}", some_new_value);

    let some_new_value_2 = unwrap_or_else(None::<f32>, || Default::default());
    println!("Some new values 2  : {}", some_new_value_2);

    let some_int = unwrap_or_default(Some(42_i32));
    let none_string = unwrap_or_default(None::<String>);
    let none_vec = unwrap_or_default(None::<Vec<i32>>);
    println!("{}, {}, {:?}", some_int, none_string, none_vec);
}
