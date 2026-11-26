pub fn lucas_number(n: u32) -> u32 {
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    }

    let mut a = 2;
    let mut b = 1;
    let mut c;

    for _ in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    b
}



fn main() {
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        4,
        lucas_number(4)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        5,
        lucas_number(5)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        10,
        lucas_number(10)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        13,
        lucas_number(13)
    );
}
