pub fn run(n: Vec<i32>) -> usize {
    let mut idx = 0;
    let n_len = n.len();
    for i in 0..n_len {
        if n[idx] < n[i] {
            idx = i
        }
    }
    return idx;
}
