/*
 * Using a hashmap and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, "Add Sally to Engineering" or "Add Amir to Sales."
 *
 * Then let the users retrieve a list of all people in a department or all people in the company by department, sorted alphabetically
 */

use std::collections::HashMap;
use std::io;

fn parse_command(command: &str) -> (String, String) {
    let words: Vec<&str> = command.split_whitespace().collect();
    // "Add <name> to <dept>"
    (words[1].to_string(), words[3].to_string())
}

fn main() {
    println!("Add employees to departments");

    let mut mp: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nEnter your choice\n1. Add Employee to a department\n2. Print all by department\n3. Print a department\n4. Quit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Please enter a valid choice");

        match choice.trim() {
            "1" => {
                let mut command = String::new();
                println!("Template: Add Sarah to Sales");
                println!("Enter your command:");
                io::stdin()
                    .read_line(&mut command)
                    .expect("Please enter a valid value");

                let (name, dept) = parse_command(command.trim());
                mp.entry(dept.clone())
                    .or_insert_with(Vec::new)
                    .push(name.clone());
                println!("Added {} to {}", name, dept);
            }
            "2" => {
                if mp.is_empty() {
                    println!("No employees yet.");
                } else {
                    let mut departments: Vec<&String> = mp.keys().collect();
                    departments.sort();
                    for dept in departments {
                        let mut employees = mp[dept].clone();
                        employees.sort();
                        println!("{}: {}", dept, employees.join(", "));
                    }
                }
            }
            "3" => {
                println!("Enter department name:");
                let mut dept = String::new();
                io::stdin().read_line(&mut dept).expect("Failed to read");
                let dept = dept.trim();
                match mp.get(dept) {
                    Some(employees) => {
                        let mut sorted = employees.clone();
                        sorted.sort();
                        println!("{}: {}", dept, sorted.join(", "));
                    }
                    None => println!("Department '{}' not found.", dept),
                }
            }
            "4" => break,
            _ => println!("Invalid choice -_- Try Again"),
        }
    }
}
