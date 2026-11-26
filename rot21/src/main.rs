pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let a = 'a' as u8;
                let z = 'z' as u8;
                let c_u8 = c as u8;
                let rotated = (c_u8 - a + 21) % 26 + a;
                rotated as char
            } else if c.is_ascii_uppercase() {
                let a = 'A' as u8;
                let z = 'Z' as u8;
                let c_u8 = c as u8;
                let rotated = (c_u8 - a + 21) % 26 + a;
                rotated as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher will be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher will be: {}", rot21("rot21 works!"));
}
