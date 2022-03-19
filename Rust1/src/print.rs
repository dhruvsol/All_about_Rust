pub fn run() {
    println!("hello world print rs");
    //  Basic formating
    println!("Number:{}", 2);
    println!("{} is {}", "blaze", "tea");
    // Positional Arguments
    println!(
        "{0} is from {1} and {0} like to {2}",
        "dhruv", "india", "code"
    );
    // Name arguments
    println!("{name} like to {act}", name = "dhruv", act = "code");
    // PlaceHolder traits
    println!("Binary:{:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    // Place holder for debug
    println!("{:?}", (12, false, "test"))
}
