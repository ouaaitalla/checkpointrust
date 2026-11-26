pub fn next_prime(nbr: usize) -> usize {
    let mut num = nbr;
    while num > 1 {
        let mut is_prime = true;
        for i in 2..=((num as f64).sqrt() as usize) {       
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            return num;
        }
        num += 1;
    }
    0 // Return 0 if no next prime is found
}

fn main() {
    println!("The next prime after 4 is: {}", next_prime(4));
    println!("The next prime after 11 is: {}", next_prime(11));
}