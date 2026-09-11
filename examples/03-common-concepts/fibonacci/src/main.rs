use std::io;

fn main() {
    println!("Enter n");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: u32 = n.trim().parse().expect("Please type a valid number");

    let a = 0;
    let b = 1;

    let mut count: u32 = 1;

    println!("The fibonacci series is as follows:");
    println!("0\n1");
    print_fibonacci(a, b, n, &mut count);
}

fn print_fibonacci(a: u32, b: u32, n: u32, count: &mut u32) -> () {
    let c = a + b;

    println!("{}", c);
    *count += 1;

    if *count != n + 1 {
        print_fibonacci(b, c, n, count);
    }
}
