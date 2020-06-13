pub fn run() -> i32 {
    fibonacci(3)
}

pub fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}
