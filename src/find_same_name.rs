use std::collections::HashSet;

pub fn run(n: Vec<&str>) -> HashSet<&str> {
    let mut results = HashSet::new();
    let len = n.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            if n[i] == n[j] {
                results.insert(n[i]);
            }
        }
    }
    return results;
}
