pub fn run() -> i32 {
    recursive_factorial(5)
}

fn recursive_factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    return n * recursive_factorial(n - 1);
}
