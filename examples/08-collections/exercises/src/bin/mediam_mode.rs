use std::collections::HashMap;
use std::convert::TryInto;
use std::io;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut size = String::new();

    println!("Enter the size of the vector");
    io::stdin()
        .read_line(&mut size)
        .expect("Please enter a valid size");

    // Shadowing the size variable
    let size: i32 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            println!("Program exited");
            std::process::exit(0); // 0-> sucess, non-zero is error
        }
    };

    println!("Please enter the vector in sorted manner");
    for i in 0..size {
        let pos = match i {
            0 => "st",
            1 => "nd",
            2 => "rd",
            _ => "th",
        };
        println!("Enter the {}{} element", i + 1, pos);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a valid input");

        let num: i32 = input.trim().parse().unwrap();
        v.push(num);
    }

    println!("The vector is : {v:?}");

    println!("Calculating median and mode");

    if size % 2 == 0 {
        // even sized array, two middle elements
        let mid: usize = (size / 2).try_into().unwrap();
        let median: f32 = (v[mid] + v[mid - 1]) as f32 / 2.0;

        println!("The median is {median}");
    } else {
        // odd sized array
        let mid: usize = (size / 2).try_into().unwrap();
        let median = v[mid];
        println!("The median is {median}");
    }

    // Creating hashmap for mode
    let mut mp = HashMap::new();

    for num in &v {
        let count = mp.entry(*num).or_insert(0);
        *count += 1;
    }

    let mut max_key = 0;
    let mut max_count = 0;

    for (key, value) in &mp {
        if value > &max_count {
            max_count = *value;
            max_key = *key;
        }
    }

    if mp.len() == v.len() {
        println!("All the elements are unique and there is no mode");
        std::process::exit(0);
    }

    println!("The mode of the vector is: {max_key}");
}
