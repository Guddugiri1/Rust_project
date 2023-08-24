//Using this source code make a simple calculator
//And visit on GitHub Profile for free source code

use std::io;

fn main() {
    loop {
        println!("Calculator Menu:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Quit");

        let choice: i32 = get_input("Enter your choice:");

        if choice == 5 {
            println!("Goodbye!");
            break;
        }

        let num1: f64 = get_input("Enter the first number:");
        let num2: f64 = get_input("Enter the second number:");

        match choice {
            1 => println!("Result: {}", num1 + num2),
            2 => println!("Result: {}", num1 - num2),
            3 => println!("Result: {}", num1 * num2),
            4 => {
                if num2 != 0.0 {
                    println!("Result: {}", num1 / num2);
                } else {
                    println!("Error: Cannot divide by zero");
                }
            },
            _ => println!("Invalid choice"),
        }
    }
}

fn get_input<T>(message: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid value."),
        }
    }
}
