use std::io;

fn read_input() -> f64 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap() as f64;
    return number.trim().parse().unwrap();
}

fn read_operators() -> String {
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    return operator.trim().to_string();
}

fn operate(num1: f64, num2: f64, operator1: String) -> f64 {
    match operator1.as_str() {
        "plus" => num1 + num2,
        "minus" => num1 - num2,
        "multiply" => num1 * num2,
        "division" => {
            if num2 == 0.0 {
                println!("error : can't divide by zero");
                0.0
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("invalid option");
            0.0
        }
    }
}

fn main() {
    loop {
        println!("please enter first number : ");
        let number1 = read_input();

        println!("please enter second number : ");
        let number2 = read_input();

        println!(
            "please enter operator : available operators are : plus, minus, multiply, division"
        );
        let operator = read_operators();

        let result = operate(number1, number2, operator.to_lowercase());
        println!("result {}", result);
    }
}
