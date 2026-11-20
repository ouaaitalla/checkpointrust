pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    let lower_letter = letter.to_ascii_lowercase();
    s.chars().filter(|&c| c.to_ascii_lowercase() != lower_letter).collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars().map(|c| {
        if c == letter {
            c.swapcase()
        } else {
            c
        }
    }).collect()
}


fn main() {
    println!("{}", remove_letter_sensitive("Jojhn jis sljeepjjing", 'j'));
    println!(
        "{}",
        remove_letter_insensitive("JaimA ais swiaAmmingA", 'A')
    );
    println!("{}", swap_letter_case("byE bye", 'e'));
}
