
use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let  words = words;
    let mut word_count:HashMap<String, u32> = HashMap::new();
    for word in     words.split_whitespace() {
        let cleaned_word = word.chars().filter(|c: &char| c.is_ascii_alphanumeric() || *c == '\'')
            .collect::<String>()
            .to_lowercase().trim_matches('\'').to_string();
        if !cleaned_word.is_empty() {
            *word_count.entry(cleaned_word).or_insert(0) += 1;
        }
    }
    word_count
}   

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and' I'm not sure about the universe.”
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}
