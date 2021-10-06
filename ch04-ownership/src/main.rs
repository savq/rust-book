fn main() {
    let s = String::from("Hello"); // s is a String: A pointer in the stack,
                                   // that points to some bytes in the heap

    let new_s = s.clone(); // Strings can't be copied, but can be cloned

    takes_ownership(s); // s is moved, so it goes out of scope here

    // Can't do anything with s here. It's not in this scope anymore
    // But new_s _can_ be used, it's still in scope
    println!("{}", new_s);

    let x = 5; // x is an integer (a value on the stack). Integers can be copied

    makes_copy(x);

    println!("{}", x); // x can still be printed, it's still in scope
} // x goes out of scope here. new_s also goes out of scope here

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string would be dropped here, and the memory on the heap deallocated

fn makes_copy(n: i32) {
    println!("{}", n);
} // n goes out of scope, so it would be popped from the stack
