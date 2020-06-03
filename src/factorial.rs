pub fn run() -> i32 {
    factorial(5)
}

fn factorial(n: i32) -> i32 {
    let mut results = 1;
    for i in 1..n + 1 {
        results = results * i
    }
    return results;
}
