pub fn run() -> i32 {
    recursive_sum(9)
}

fn recursive_sum(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return n + recursive_sum(n - 1);
}
