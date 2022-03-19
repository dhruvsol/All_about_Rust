pub fn run() {
    // Primitive str = Immutable fixed-length string somewhere in memory
    let hello = "hello";
    // String = Growable, heap-allocated data structure - Use when you need to modify or own string data
    let mut hel = String::from("HEllo ");

    // lenght of string
    println!("Lenght: {} ", hello.len());

    // To add only a char at the end
    hel.push('w');
    println!("{}", hel);
    // to add String
    hel.push_str("world");
    println!("{}", hel);

    // Capacity in bytes
    println!("Capacity: {}", hel.capacity());

    // is empty
    println!("Is empty: {} ", hel.is_empty());
    // Contains
    println!("Contains 'World' {}", hel.contains("World"));

    // Replace
    println!("Replace: {}", hel.replace("World", "There"));

    // Loop through the string
    for word in hel.split_whitespace() {
        println!("{}", word)
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('f');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
