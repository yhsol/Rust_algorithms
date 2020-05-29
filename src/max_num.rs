pub fn run(n: Vec<i32>) -> i32 {
    let len = n.len();
    let mut max = n[0];
    for i in 0..len {
        if max < n[i] {
            max = n[i]
        }
    }
    return max;
}
