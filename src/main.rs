use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    let num1 = read_number("Enter the first number:");
    let operator = read_operator("Enter an operator (+, -, *, /):");
    let num2 = read_number("Enter the second number:");

    let result = match operator.as_str() {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("Error: Unknown operator.");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operator, num2, result);
}

fn read_number(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_operator(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let operator = input.trim().to_string();
        if ["+", "-", "*", "/"].contains(&operator.as_str()) {
            return operator;
        } else {
            println!("Please enter a valid operator (+, -, *, /).");
        }
    }
}