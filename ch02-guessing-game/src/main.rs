use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    //println! is a macro, that's why it has a `!' at the end

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //thread_rng creates a random number generator
    //gen_range returns a random number in range [n, m)

    loop { // loops forever, the compiler treats it differently than `while true'
        println!("Please input your guess.");

        let mut guess = String::new(); //guess is a mutable variable. It's initially empty

        io::stdin().read_line(&mut guess) //read_line takes a mutable reference to guess, and changes it's value
            .expect("Failed to read line"); //read_line returns a Result value that has to be handled

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //the underscore is a catchall variable
        };
        //To compare guess with secret_number, they have to be the same type
        //Rust allows variable redefinitions, this is called *shadowing*
        //We don't want errors to crash the program, so we use a match

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        //a match takes a value and compares it to the possible values of an enum
    }
}

