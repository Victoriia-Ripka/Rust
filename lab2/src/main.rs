use std::io::stdin;

fn main() {
    println!("Calculator program\n\n");

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut choice = String::new();

        println!("Write first number (or type 'Exit' or 'e' to quit): ");
        stdin().read_line(&mut num1).expect("Error in input");
        let num1 = num1.trim();
        
        if num1.eq_ignore_ascii_case("exit") || num1.eq_ignore_ascii_case("e") {
            println!("Exiting the calculator. Goodbye!");
            break;
        }
        
        let num1: f64 = match num1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number, please try again.");
                continue;
            }
        };

        println!("Write second number: ");
        stdin().read_line(&mut num2).expect("Error in input");
        
        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number, please try again.");
                continue;
            }
        };

        println!("Provide the operation (+ - * /): ");
        stdin().read_line(&mut choice).expect("Error in input");

        let choice = choice.trim();
        let result = if choice == "+" {
            num1 + num2
        } else if choice == "-" {
            num1 - num2
        } else if choice == "*" {
            num1 * num2
        } else if choice == "/" {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Cannot divide by zero.");
                continue;
            }
        } else {
            println!("Not a valid choice");
            continue;
        };

        println!("{} {} {} = {}", num1, choice, num2, result);
    }
}
