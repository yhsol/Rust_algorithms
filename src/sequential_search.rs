pub fn run() {
    let list = [39, 14, 67, 105].to_vec();
    let name = ["Justin", "John", "Mike", "Summer"].to_vec();
    let index = 67;
    println!("{:?}", search_by_index(list, index, name))
}

fn sequential_search(a: Vec<i32>, s: i32) -> Vec<usize> {
    let n = a.len();
    let mut results = vec![];
    for i in 0..n {
        if a[i] == s {
            results.push(i)
        }
    }

    return results;
}

fn search_by_index(a: Vec<i32>, s: i32, n: Vec<&str>) -> &str {
    let key = sequential_search(a, s)[0];
    return n[key];
}
