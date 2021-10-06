// § 3.5. Control Flow: If statements

fn main() {
    let number = 6;

    // Using if-else
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 { //Rust will evaluate this line
        println!("Number is divisible by 3");
    } else if number % 2 == 0 { //Rust WON'T evaluate this line, since a previous condition was true
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is {}", number);
}

