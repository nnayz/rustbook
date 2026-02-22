fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(5, 10);

    let sum = sum(5, 10);
    println!("Using another function: {}", sum);

    print_labeled_measurement(5, 'h');

    let a = println!("Hello"); // THis is a macro, and macros are expressions and expressions return something, println! returns empty tuple ()
    println!("THe value of a is {:?}", a);
}

fn another_function() {
    // Use snake_case for function names, just like in Python
    println!("Another function.");
}

fn another_function_with_parameters(x: i32, y: i32) {
    println!("The sum of the numbers is {}", x + y);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y // Notice: there is no semicolon here, because this is an expression that returns a value and not a statement
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}
