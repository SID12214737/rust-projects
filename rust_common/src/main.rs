use std::io;

fn main() {
    // FizzBuzz
    println!("Fizz Buzz");
    let mut x = String::new();
    println!("Please enter number: ");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line!");

    let x: u32 = x
        .trim()
        .parse()
        .expect("Please enter a valid number!");

    if x % 3 == 0 && x % 5 == 0 {
        println!("FizzBuzz");
    } else if x % 3 == 0 {
        println!("Fizz");
    } else if x % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{x}");
    }

}
