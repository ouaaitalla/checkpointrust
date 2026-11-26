pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut steps = 0;
    let mut current_factorial = 1;
    if  factorial <= 1 {
        return steps;
    }
    while current_factorial < factorial {
        steps += 1;
        current_factorial *= steps;
    }
    if current_factorial == factorial {
        steps
    }else {
        0
    }
}

fn main() {
    println!(
        "The factorial steps of 720 = {}",
        count_factorial_steps(720)//6
    );
    println!("The factorial steps of 13 = {}", count_factorial_steps(13));//0
    println!("The factorial steps of 6 = {}", count_factorial_steps(6));//3
}