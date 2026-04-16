use anyhow::{Context, Result};
use std::io;

fn read_input() -> Result<f64> {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .context("Failed to read the stdin")?;
    let parsed = number.trim().parse::<f64>().context("Enter valid number")?;
    Ok(parsed)
}

fn read_operators() -> Result<String> {
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .context("Failed to read stdin")?;
    let operate = operator.trim().to_string();
    Ok(operate)
}

fn operate(num1: f64, num2: f64, operator1: String) -> Result<f64> {
    match operator1.as_str() {
        "plus" => Ok(num1 + num2),
        "minus" => Ok(num1 - num2),
        "multiply" => Ok(num1 * num2),
        "division" => {
            if num2 == 0.0 {
                // println!("error : can't divide by zero");
                anyhow::bail!("cannot divide by zero");
            } else {
                Ok(num1 / num2)
            }
        }
        _ => {
            // println!("invalid option");
            anyhow::bail!("Invalid option")
        }
    }
}

fn main() -> Result<()> {
    loop {
        println!("please enter first number : ");
        let number1 = read_input()?;

        println!("please enter second number : ");
        let number2 = read_input()?;

        println!(
            "please enter operator : available operators are : plus, minus, multiply, division"
        );
        let operator = read_operators()?;

        let result = operate(number1, number2, operator.to_lowercase())?;
        println!("result {}", result);
    }
}
