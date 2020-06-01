pub fn run(n: Vec<i32>) -> i32 {
    let n_len = n.len();
    let mut results = n[0];
    for i in 0..n_len {
        if results > n[i] {
            results = n[i]
        }
    }
    return results;
}
