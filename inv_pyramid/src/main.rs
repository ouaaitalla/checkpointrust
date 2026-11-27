pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    let mut result = Vec::new();
    for j in (1..=i) {
        let line = " ".repeat(j);
        let lin1e = v.repeat(j);
        let res = line + &lin1e;
        result.push(res);
    }
    for j in (1..i).rev() {
        let line = " ".repeat(j);
        let lin1e = v.repeat(j);
        let res = line + &lin1e;
        result.push(res);
    }
    result
}


fn main() {
    println!("{:#?}", inv_pyramid(String::from("#"), 1));
    println!("{:#?}", inv_pyramid(String::from("a"), 2));
    println!("{:#?}", inv_pyramid(String::from(">"), 5));
    println!("{:#?}", inv_pyramid(String::from("&"), 8));//
}
