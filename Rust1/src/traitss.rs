pub fn run() {}
trait ANimals {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}
impl Bird {
    fn print_name(&self) {
        println!("{}", self.name)
    }
}

impl Animals for Bird {}
