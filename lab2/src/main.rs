use std::io::stdin;

// Перевіряємо чи символ є оператором
fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}

// Перетворюємо вираз у префіксну нотацію
fn to_prefix(expression: &str) -> Option<Vec<String>> {
    let mut operators_stack = Vec::new(); // + - * / 
    let mut numbers_stack = String::new();
    let mut prefix_stack = Vec::new(); 
    
    for c in expression.chars().rev() {
        if c.is_digit(10) || c == '.' {
            numbers_stack.push(c); // зчитуємо число з права наліво
        } else if is_operator(c) {
            if !numbers_stack.is_empty() {
                prefix_stack.push(numbers_stack.chars().rev().collect::<String>()); // Додаємо число
                numbers_stack.clear();
            }
            while let Some(&top_op) = operators_stack.last() {
                if precedence(top_op) > precedence(c) {
                    let operator = operators_stack.pop().unwrap();
                    let operand1 = prefix_stack.pop().unwrap();
                    let operand2 = prefix_stack.pop().unwrap();
                    let new_expr = format!("{} {} {}", operator, operand1, operand2);
                    prefix_stack.push(new_expr);
                } else {
                    break;
                }
            }
            operators_stack.push(c);
        } else {
            return None; // неприпустимий символ
        }
    }

    if !numbers_stack.is_empty() {
        prefix_stack.push(numbers_stack.chars().rev().collect::<String>());
    }

    while let Some(op) = operators_stack.pop() {
        let operand1 = prefix_stack.pop().unwrap();
        let operand2 = prefix_stack.pop().unwrap();
        let new_expr = format!("{} {} {}", op, operand1, operand2);
        prefix_stack.push(new_expr);
    }

    Some(prefix_stack)
}

// Визначаємо пріоритет оператора
fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

// Обчислюємо вираз у префіксній формі
fn calculate_prefix(prefix: &Vec<String>) -> Option<f64> {
    let mut stack: Vec<f64> = Vec::new();

    for token in prefix.iter().rev().flat_map(|s| s.split_whitespace().rev()) {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else if is_operator(token.chars().next().unwrap()) {
            if stack.len() < 2 {
                return None; // Недостатньо операндів
            }
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            let result = match token {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        println!("Помилка: ділення на нуль.");
                        return None;
                    } else {
                        a / b
                    }
                }
                _ => return None, // Неприпустимий оператор
            };
            stack.push(result);
        }
    }

    stack.pop()
}

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

        // Перевіряємо та перетворюємо вираз у префіксну нотацію
        match to_prefix(trimmed_input) {
            Some(prefix) => {
                println!("Префікс: {:?}", prefix);

                // Обчислюємо значення на основі префіксного запису
                if let Some(result) = calculate_prefix(&prefix) {
                    println!("Результат: {}", result);
                    last_result = Some(result);
                } else {
                    println!("Помилка під час обчислення виразу.");
                }
            }
            None => {
                println!("Некоректний вираз, спробуйте знову.");
            }
        }
    }
}
