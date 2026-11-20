pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    h.values().min().copied().unwrap_or(1)
}


use std::collections::HashMap;

fn main() {
    let hash = HashMap::from([
        ("a", 10),
        ("b", 20),
        ("c", 5),
        ("d", -0),
        ("e", -0)
    ]);

    println!(
        "The smallest of the elements in the HashMap is {}",
        smallest(hash)
    );
}

