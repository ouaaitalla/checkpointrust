pub fn reverse_it(v: i32) -> String {
    let mut result = String::new();
    let mut num = v;

    if num < 0 {
        result.push('-');
        num = -num;
    }

    let reversed: String = num.to_string().chars().rev().collect();
    result.push_str(&reversed);
    result.push_str(&num.to_string());

    result
}

fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}