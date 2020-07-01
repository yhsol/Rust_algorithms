use std::convert::TryInto;

pub fn run() {
    let mut d = vec![2, 4, 1, 5, 3];
    println!("{:?}", selection_sort(&mut d))
}

fn find_min_idx(a: &mut Vec<i32>) -> usize {
    // println!("find_min_idx");
    let n = a.len();
    let mut min_idx = 0;

    for i in 1..n {
        if a[min_idx] > a[i] {
            min_idx = i.try_into().unwrap();
        };
    }

    return min_idx.try_into().unwrap();
}

fn selection_sort(a: &mut Vec<i32>) -> Vec<i32> {
    println!("selection_sort");
    let mut result = vec![];

    while &mut a.len() > &mut 0 {
        let min_idx = find_min_idx(&mut a.to_vec());

        a.retain(|&x| x != a[min_idx]);
        result.push(a[min_idx]);
    }
    return result;
}
