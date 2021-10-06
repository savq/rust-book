// § 3.5. Control flow: Loops
fn main() {
    // loop loops?
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Breaks return values :o just great.
        }
    };
    println!("The result is {}", result);

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");


    // For loops
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // Range and rev
    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}

