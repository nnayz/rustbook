use std::any::type_name_of_val;
use std::io;

fn main() {
    let mut x = 10;

    println!("The value of x is: {x}");

    x = 5;
    println!("The value of x is: {x}");

    // Naming conventions for constants: all caps
    // Also, data type annotation is necessary in constants, i.e., u32
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in te inner scope: {x}");
    }

    println!("The value of x is: {x}");

    // Data Types
    let guess: u32 = "42"
        .trim()
        .parse()
        .expect("Not a number, please enter a valid one instead");

    println!("The value of guess is: {guess}");

    // Integer types
    println!("| -------------------- | --------- | --------- |");
    println!("| Length               | Signed    | Unsigned  |");
    println!("| -------------------- | --------- | --------- |");
    println!("| 8 bits               | i8        | u8        |");
    println!("| 16 bits              | i16       | u16       |");
    println!("| 32 bits              | i32       | u32       |");
    println!("| 64 bits              | i64       | u64       |");
    println!("| 128 bits             | i128      | u128      |");
    println!("| arch                 | isize     | usize     |");
    println!("| -------------------- | --------- | --------- |");

    // Each signed variant can store numbers from - (2^(n - 1)) to 2^(n - 1) - 1 inclusively. n is the number of bits being used.
    //
    println!("\n\n\n");

    // Integer literals
    println!("| -------------------- | --------- |");
    println!("| Number literals      | Example   |");
    println!("| -------------------- | --------- |");
    println!("| Decimal              | 98_222    |");
    println!("| Hex                  | 0xff      |");
    println!("| Octal                | 0o77      |");
    println!("| Binary               | 0b1111_000|");
    println!("| Byte (u8 only)       | b'A'      |");
    println!("| -------------------- | --------- |");

    println!("\n\n\n");

    // Floating-point types
    println!("| -------------------- | --------- | --------- |");
    println!("| 32 bits              | f32       |           |");
    println!("| 64 bits              | f64       |           |");
    println!("| -------------------- | --------- | --------- |");

    // When compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.
    //

    // Floating point numbers
    // by default the data type is f64
    //

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let sum = x + y;

    println!("Sum of x and y: {}", sum);
    println!("And the type is: {}", type_name_of_val(&sum)); // By default the type is f32

    // boolean tyeps
    let t = true;
    let f: bool = false; // type annotation

    println!("And the type is: {}", type_name_of_val(&t));
    println!("And the type is: {}", type_name_of_val(&f));

    // char types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation 4 bytes in size
    let heart_eyed_cat = '😻';

    println!("And the type is: {}", type_name_of_val(&c));
    println!("And the type is: {}", type_name_of_val(&z));
    println!("And the type is: {}", type_name_of_val(&heart_eyed_cat));

    // Compound types
    // Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("And the type is: {}", type_name_of_val(&tup));

    let (x, y, z) = tup; // destructuring

    println!("And the type is: {}", type_name_of_val(&x));
    println!("And the type is: {}", type_name_of_val(&y));
    println!("And the type is: {}", type_name_of_val(&z));

    // Also using this .

    let five_hundred = tup.0;
    println!("Five hundred: {}", five_hundred);

    // Tuple without any values has a special name, unit. This value and its corresponding type are both written as (), and represents an empty value or an empty return type.

    // Array
    let a = [1, 2, 3, 4, 5];

    // Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far. Also array is not as flexible as the vector type.
    //
    //  Array more useful is you alrady know the length of the array
    //
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("And the type is: {}", type_name_of_val(&months));

    // Accessing array elements
    let first_month = months[0];
    println!("First month: {}", first_month);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; // [ 3, 3, 3, 3, 3 ]

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!"); // thread 'main' will panick if the index provided is greater than or equal to the length of the array.

    println!("The data type of index: {}", type_name_of_val(&index)); // prints usize

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
