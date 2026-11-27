use std::env;

fn evaluate(expression: &str) -> Option<i64> {
    let mut stack: Vec<i64> = Vec::new();

    for token in expression.split_whitespace() {
        if let Ok(number) = token.parse::<i64>() {
            stack.push(number);
        } else {
            let b = stack.pop()?;
            let a = stack.pop()?;

            let result = match token {
                "+" => a.checked_add(b)?,
                "-" => a.checked_sub(b)?,
                "*" => a.checked_mul(b)?,
                "/" => a.checked_div(b)?,
                "%" => a.checked_rem(b)?,
                _ => return None,
            };

            stack.push(result);
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error");
        return;
    }

    match evaluate(&args[1]) {
        Some(result) => println!("{}", result),
        None => println!("Error"),
    }
}