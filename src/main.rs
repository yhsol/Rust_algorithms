mod max_index;

fn main() {
    println!("Hello, world!");
    let list = vec![1, 5, 3, 5, 9];
    println!("{}", max_index::run(list))
}
