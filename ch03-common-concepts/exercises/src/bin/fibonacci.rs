// Rustbook ch. 03, exercise 2: Generate the nth Fibonacci number.
// NOTE: fibonacci(186) < 2^128 < fibonacci(187)

use std::io;

fn main() {
    loop {
        println!("Write a number:");

        let mut input_num = String::new();
        io::stdin()
            .read_line(&mut input_num)
            .expect("Failed to read line"); // Handle Result

        // Shadow option
        let input_num: u32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("select a valid option");
                continue;
            }
        };

        println!("Fibonacci({}) = {}", input_num, fibo(input_num));
    }
}

fn fibo(n: u32) -> u128 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 0;

    for _ in 1..n {
        a = b + c;
        c = b;
        b = a;
    }
    return a;
}
