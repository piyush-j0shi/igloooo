// can only pass the same things for the same kind of expected closures
// if try to pass fn mut for a function which expects fn we get cannot borrow any_value as mutable,
// as it is a captured variable in a Fn closure
// or if I try to pass fnmut to FnOnce to the FnMut you get cannot move out of value, a captured variable in an `FnMut` closure
// you might want to use the .clone() or if the value implements the copy trait then it can be
// helped

fn first_higher_order_function<F>(method: F)
where
    F: Fn(),
{
    method();
    method()
}

fn second_higher_order_function<G>(mut method: G)
where
    G: FnMut(),
{
    method();
    method();
}

fn third_higher_order_function<H>(method: H)
where
    H: FnOnce(),
{
    method()
}

fn main() {
    let onething = "onething";
    let mut anotherthing = String::from("Heelo");
    let something = String::from("something");

    first_higher_order_function(|| {
        println!(
            "this is the first higher order function and the value is : {}",
            onething
        )
    });
    second_higher_order_function(|| {
        anotherthing.push_str("heloo");
        anotherthing.push_str("hello");
        println!("anotherthing : {}", anotherthing);
    });
    third_higher_order_function(|| {
        drop(something);
    });

    // second_higher_order_function(|| {
    // drop(anotherthing);
    // });

    // first_higher_order_function(|| {
    // anotherthing.push_str("something");
    // });
}
