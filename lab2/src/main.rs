use std::io::stdin;
use std::collections::VecDeque;

// Перевіряємо чи символ є оператором
fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/')
}

// Визначаємо пріоритет оператора
fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

// Перетворюємо вираз у префіксну нотацію
fn to_prefix(expression: &str) -> Option<Vec<String>> {
    let mut operators_stack = VecDeque::new(); // + - * / 
    let mut numbers_stack = String::new();
    let mut prefix_stack = VecDeque::new(); 
    
    for c in expression.chars().rev() {
        if c.is_digit(10) || c == '.' {
            numbers_stack.push(c); // зчитуємо число з права наліво
        } else if is_operator(c) {
            if !numbers_stack.is_empty() {
                prefix_stack.push_front(numbers_stack.chars().rev().collect::<String>()); // додаємо число
                numbers_stack.clear();
            }
            while let Some(&top_op) = operators_stack.back() {
                if precedence(top_op) > precedence(c) {
                    let operator = operators_stack.pop_back().unwrap();
                    let operand2 = prefix_stack.pop_front().unwrap();
                    let operand1 = prefix_stack.pop_front().unwrap();
                    let new_expr = format!("{} {} {}", operator, operand1, operand2);
                    prefix_stack.push_front(new_expr);
                } else {
                    break;
                }
            }
            operators_stack.push_back(c);
        } else {
            return None; // неприпустимий символ
        }
    }

    if !numbers_stack.is_empty() {
        prefix_stack.push_front(numbers_stack.chars().rev().collect::<String>());
    }

    while let Some(op) = operators_stack.pop_back() {
        let operand2 = prefix_stack.pop_front().unwrap();
        let operand1 = prefix_stack.pop_front().unwrap();
        let new_expr = format!("{} {} {}", op, operand1, operand2);
        prefix_stack.push_front(new_expr);
    }

    Some(prefix_stack.into_iter().collect())
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
            }
            None => {
                println!("Некоректний вираз, спробуйте знову.");
            }
        }
    }
}
