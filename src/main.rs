mod max_num;

fn main() {
    println!("Hello, world!");
    let list = vec![1, 5, 3, 5, 9];
    println!("{}", max_num::run(list))
}
