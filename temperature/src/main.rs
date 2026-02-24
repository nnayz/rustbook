use std::io;

fn main() {
    println!("Enter the temp scale:\n1. Fahrenheit\n2. Celsius");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Invalid choice");

    let choice: i32 = choice.trim().parse().expect("Invalid choice");

    let mut temp = String::new();

    println!("Enter temperature to convert");

    io::stdin()
        .read_line(&mut temp)
        .expect("Invalid temperature");

    let temp: f64 = temp.trim().parse().expect("Invalid temperature");

    if choice == 1 {
        // Fahrenheit to Celcius
        let result: f64 = ((temp - 32.0) * 5.0) / 9.0;
        println!("The equivalent temperature: {:.2} C", result);
    } else if choice == 2 {
        // Celcius to Fahrenheit
        let result: f64 = (temp * 9.0 / 5.0) + 32.0;
        println!("The equivalent temperature: {:.2} F", result);
    } else {
        println!("Invalid choice!");
    }
}
