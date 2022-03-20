pub fn run() {
    let i = 5;
    match i {
        0 => println!("the number is {}", i),
        1 | 2 => println!("the number is {}", i),
        3..=4 => println!("the number is {}", i),
        _ => println!("the number is "),
    }
}
