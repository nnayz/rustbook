fn main() -> () {
    // If expressions
    //
    let number = 3;

    if number < 5 {
        // no parenthesis
        // Has Arms
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Condition based assignment statement
    let condition = true;

    // let number = if condition { 5 } else { "six" }; // Will throw an error

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Loop
    // 3 types of loops
    // 1. loop
    // 2. while
    // 3. for
    //
    // loop is infinite loop unless break

    let mut counter = 10;

    let result = loop {
        // loop can return values
        if counter == 0 {
            break counter;
        }
        counter -= 1;
    };

    println!("Result: {}", result);

    // Disambiguation using loop labels
    // Giving loop names
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    println!("====================");

    // Looping through an array using a while loop
    let arr = [1, 2, 3, 4, 5];

    let mut index = 0;
    while index < arr.len() {
        println!("[while] The value is: {}", arr[index]);
        index += 1;
    }

    println!("=====================");

    // Using for loop (faster: compiler does not add runtime code to perform conditional check of whether the index is within bounds)

    for element in arr {
        println!("[for] The value is: {}", element);
    }

    // Reverse loop (1..4) = [1, 2, 3] and (1..=4) = [1, 2, 3, 4]
    for number in (1..=4).rev() {
        println!("[reverse] The value is {}", number);
    }

    println!("LIFTOFF!!!");
}
