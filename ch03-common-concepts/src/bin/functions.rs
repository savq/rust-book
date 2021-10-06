// § 3.3. Functions
fn main() {
    another_function(5, 6);

    let six = plus_one(5);
    println!("The value of x is: {}", six);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
