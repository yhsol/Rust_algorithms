mod find_same_name;
// mod min_num;

fn main() {
    println!("Hello, world!");
    let list = vec!["tom", "jerry", "tom", "jerry"];
    println!("{:?}", find_same_name::run(list))
}
