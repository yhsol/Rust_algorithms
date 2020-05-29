pub fn run(n: i32) -> i32 {
    let mut result = 0;
    for i in 1..n + 1 {
        result = result + (i * i);
    }

    return result;
}
