pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(arr.len() + 1);
    let mut sum = arr.iter().sum();
    result.push(sum);
    for &num in arr.iter().rev() {
        sum -= num;
        result.push(sum);
    }
    result
}

fn main() {
    println!(
        "Partial sums of [5, 18, 3, 23] is: {:?}",
        parts_sums(&[5, 18, 3, 23])
    );//[49, 26, 23, 5, 0]
}
