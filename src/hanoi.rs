pub fn run() {}

pub fn hanoi(n: u32, start: u32, middle: u32, end: u32) {
    if n == 1 {
        println!("{} -> {}", start, end);
        return;
    }
    hanoi(n - 1, start, end, middle);
    println!("start1: {}", start);
    println!("end1: {}", end);
    println!("{} -> {}", start, end);
    println!("start2: {}", start);
    println!("end2: {}", end);
    hanoi(n - 1, middle, start, end);
}
