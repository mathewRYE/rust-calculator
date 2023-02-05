use std::io;
fn main() {
    println!("Welcome to my simple Rust calculator, capable of any simple arithmetic operations!");


loop {
    println!( "Enter the expression you would like to evaluate, add a space between the number and operation: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Input could not be read, please try again.");

    let input = input.trim();
    if input == "Exit"{
        break;
    }

    let result = match input.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            let parts: Vec<&str> = input.split(" ").collect();
                if parts.len() != 3 {
                    println!("Invalid input, please try again.");
                    continue;
                }

                let num1 = parts[0].parse::<f64>().unwrap();
                let num2 = parts[2].parse::<f64>().unwrap();
                let operator = parts[1];

                match operator {
                    "+" => num1 + num2,
                    "-" => num1 - num2,
                    "*" => num1 * num2,
                    "/" => num1 / num2,
                    _ => {
                        println!("Invalid operator, please try again.");
                        continue;
                    }
                }
            }
        };
        println!("Result: {}", result);
    }
}