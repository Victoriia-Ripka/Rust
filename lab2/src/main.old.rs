use std::io::stdin;

fn main() {
    println!("Калькулятор\n\n");

    let mut last_result: Option<f64> = None;

    loop {

        let mut input = String::new();

        // Показуємо попередній результат, якщо доступний
        if let Some(prev_result) = last_result {
            println!("Попередній результат: {}.", prev_result);
        }

        // Вихід з циклу
        println!("Введіть вираз (або 'вихід'/'в' для виходу): ");
        stdin().read_line(&mut input).expect("Помилка вводу");
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("вихід") || trimmed_input.eq_ignore_ascii_case("в") {
            println!("На все добре!");
            break;
        }

        // Розділяємо ввід на частини
        let tokens: Vec<&str> = trimmed_input.split_whitespace().collect();
        if tokens.len() < 3 {
            println!("Будь ласка, введіть коректний вираз у форматі: оператор перше число друге число.");
            continue;
        }

        // Зчитуємо оператор та операнди
        let choice = tokens[0];
        let num1: f64 = match tokens[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Неправильне значення для першого числа, спробуйте ще.");
                continue;
            }
        };

        let num2: f64 = match tokens[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Неправильне значення для другого числа, спробуйте ще.");
                continue;
            }
        };

        // Виконуємо операцію
        let result = if choice  == "+" {
            num1 + num2
        } else if choice == "-" {
            num1 - num2
        } else if choice == "*" {
            num1 * num2
        } else if choice == "/" {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Помилка: ділення на нуль.");
                continue;
            }
        } else {
            println!("Недопустиме значення операції");
            continue;
        };

        println!("{} {} {} = {}", num1, choice, num2, result);

        // Зберігаємо результат для наступного використання
        last_result = Some(result); 
    }
}