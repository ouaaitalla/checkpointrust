pub fn prev_prime(nbr: u64) -> u64  {
    let mut num = nbr - 1;
    while num > 1 {
        let mut is_prime = true;
        for i in 2..=((num as f64).sqrt() as u64) {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            return num;
        }
        num -= 1;
    }
    0 // Return 0 if no previous prime is found (for nbr <= 2)
}

fn main() {
    println!("The previous prime number before 34 is: {}", prev_prime(34));
}