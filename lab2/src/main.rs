use std::io::stdin;

fn main() {
    println!("Калькулятор\n\n");

    let mut last_result: Option<f64> = None;

    loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut choice = String::new();

        // Використовуємо попередній результат, якщо доступний
        // і запитуємо ввід чисел від користувача
        if let Some(prev_result) = last_result {
            println!("Попередній результат: {}. Використати його як перше число? (т/н): ", prev_result);
            let mut use_last = String::new();
            stdin().read_line(&mut use_last).expect("Помилка вводу");
            
            if use_last.trim().eq_ignore_ascii_case("т") {
                num1 = prev_result.to_string();
            } else {
                println!("Введіть перше число (або 'вихід'/'в' для виходу): ");
                stdin().read_line(&mut num1).expect("Помилка вводу");
                if num1.trim().eq_ignore_ascii_case("вихід") || num1.trim().eq_ignore_ascii_case("в") {
                    println!("На все добре!");
                    break;
                }
            }
        } else {
            println!("Введіть перше число (або 'вихід'/'в' для виходу): ");
            stdin().read_line(&mut num1).expect("Помилка вводу");
            if num1.trim().eq_ignore_ascii_case("вихід") || num1.trim().eq_ignore_ascii_case("в") {
                println!("На все добре!");
                break;
            }
        }

        let num1: f64 = match num1.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Неправильне значення, спробуйте ще.");
                continue;
            }
        };

        println!("Введіть друге число: ");
        stdin().read_line(&mut num2).expect("Помилка вводу");
        
        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Неправильне значення, спробуйте ще.");
                continue;
            }
        };
        
        // Зчитуємо оператор та операнди
        println!("Введіть операцію (+ - * /): ");
        stdin().read_line(&mut choice).expect("Помилка вводу");

        // Виконуємо операцію
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