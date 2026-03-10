fn identify<T>(x: T) -> T {
    return x;
}

fn clone_it<T: Clone>(x: &T) -> T {
    let owned_value: T = x.clone();
    return owned_value;
}

fn main() {
    let identify_value = identify(5);
    let clone_it_value = clone_it(&identify_value);

    println!(
        "original value : {}, clone_it value : {}",
        identify_value, clone_it_value
    );
}
