fn main() {
    // for printing a line
    println!("{}", "Hello world");
    //to assert if 2 things are equal
    let a = 22;
    // assert_eq! will return panic if they are not equal
    assert_eq!(a, 22);

    // loops in rust

    for i in 0..5 {
        //0..5 is the range where last number is
        //not included
        if i % 2 == 0 {
            println!("even: {}", i);
        } else {
            println!("Odd: {}", i);
        }
    }
    // other way of writing it
    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{} {}", even_odd, i);
    }
    // for adding something can't do normal way we have to convert it to float manully rust won't convert it automatically. example
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum:{}", sum);
    ////////////////////////
    //function are type explicit
    fn sqr(x: f64) -> f64 {
        return x * x;
    }
    let res = sqr(4.0);
    println!("sqare is {}", res);
    // we don't use return in the functions because the body of the function contains the value of last expression
    fn srqnew(x: f64) -> f64 {
        x * x
    }
    let res1 = srqnew(2.0);
    println!("square new: {} ", res1);
    // passing value with refernce
    fn by_ref(x: &i32) -> i32 {
        *x + 1
    }
    let ix = 2;
    let resi1 = by_ref(&ix);
    let resi2 = by_ref(&22);
    println!("the value are: {} {}", resi1, resi2);

    // Arrays & Slices

    let arr = [1, 2, 3, 4, 5];
    // let first = arr[0];
    for i in 0..arr.len() {
        println!("[{}] = {}", i, arr[i]);
    }
    // slicing
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("ints:{:?}", ints);
    println!("slice1:{:?}", slice1);
    println!("slice2:{:?}", slice2);
    let first = &ints.get(1);
    let last = &ints.get(7);
    println!("first:{:?}, last:{:?}", first, last);
    // options has 2 type "Some" & "None"
    println!("{:?},{:?}", first.is_some(), first.is_none());
    println!("{:?},{:?}", last.is_some(), last.is_none());
    // unwrap is use to get the value of "Some" type.if use in "None" type it will throw panic
    println!("{}", first.unwrap());
}
