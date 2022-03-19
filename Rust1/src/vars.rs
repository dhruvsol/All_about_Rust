pub fn vars() {
    let name = "dhruv";
    // mut is to make variable mutable
    let mut age = 19;
    println!("My name is {} & I am {}", name, age);
    age = 20;
    println!("My name is {} & I am {}", name, age);

    // const Not use much time
    const ID: i32 = 001;

    println!("ID {}", ID);

    // multiple variable

    let (my_name, my_age) = ("dhruv", 19.5);

    println!("My name is {} and age is {}", my_name, my_age)
}
