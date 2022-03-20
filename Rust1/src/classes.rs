pub fn run() {
    let name = String::from("Bird");
    let bird = Birds {
        name: name,
        color: 2,
    };
    bird.print_name();
}
struct Birds {
    name: String,
    color: u64,
}
impl Birds {
    fn print_name(&self) {
        println!("{}", self.name)
    }
}
