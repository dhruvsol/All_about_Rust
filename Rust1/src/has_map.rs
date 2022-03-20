use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();

    map.insert(0, "helo");
    map.insert(1, "helo1");

    match map.get(&0) {
        Some(str) => println!("{}", str),
        None => println!("no value"),
    }
    match map.get(&2) {
        Some(str) => println!("{}", str),
        None => println!("no value"),
    }

    map.remove(&0);
    println!("{:?}", map);
}
