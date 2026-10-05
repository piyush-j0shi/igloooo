fn part_one() {
    let mut option_string: Option<String> = Some(String::from("some string"));

    // let length = option_string.map(|s| s.len());
    // println!("{:?}", option_string);

    let length = option_string.as_ref().map(|s| s.len());

    println!("{:?}", length);
    println!("{:?}", option_string);

    let _mutable = option_string.as_mut().map(|s| s.push_str(" this is"));
    println!("{:?}", option_string);

    let option_str = option_string.as_deref().map(|s| s.to_uppercase());
    println!("{:?}", option_str);
}

fn part_two(input_1: &Option<String>, input_2: Option<&str>) {
    // input_1.map(|s| s.len());

    let length = input_1.as_ref().map(|s| s.len());
    println!("{:?}", length);

    let length_1 = input_2.map(|s| s.len());
    println!("{:?}", length_1);
}

fn part_three() {}

fn main() {
    println!("iglooo_58.rs");
    part_one();
    part_two(&Some(String::from("this")), Some("this"));
}
