fn recursive_find_max(a: &Vec<i32>, n: usize) -> i32 {
    if n == 1 {
        return a[0];
    };

    let max_n_1 = recursive_find_max(&a, n-1);
    if max_n_1 > a[n-1] {
        return max_n_1
    } else {
        return a[n-1]
    }
}

pub fn run() {
    let v = [17,92,18,33,58,7,33,42,10];
    let result = recursive_find_max(&v.to_vec(), v.len());
    println!("{:?}", result);
}