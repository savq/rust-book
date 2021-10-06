// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then let the user retrieve a list of all people in a department or
// all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn manage_employees() {
    // Initialize hash map
    let mut company: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut menu_opt = String::new();
    loop {
        println!("1. insert\n2. view\n3. view all");
        io::stdin().read_line(&mut menu_opt).unwrap();
        let menu_num: u8 = menu_opt.trim().parse().expect("Not a number");
        match menu_num {
            1 => add_employee(&mut company),
            2 => view_dept(&company),
            _ => println!("Not an option"),
        }
    }
}

fn add_employee(company: &mut HashMap<&str, Vec<&str>>) {
    let mut name = String::new();
    let mut dept = String::new();
    println!("* Add an employee");
    print!("Name: ");
    io::stdin().read_line(&mut name).unwrap();
    print!("Department: ");
    io::stdin().read_line(&mut dept).unwrap();

    // Check department exists
    // if not, create it
    // Add employee to department
}

fn view_dept(company: &HashMap<&str, Vec<&str>>) {
    let mut dept = String::new();
    print!("* View a department:\ndepartment name: ");
    io::stdin().read_line(&mut dept).unwrap();

    if let Some(dept_employees) = company.get(&*dept) {
        println!("{:?}", dept_employees);
    };
}

fn main() {
    manage_employees();
}
