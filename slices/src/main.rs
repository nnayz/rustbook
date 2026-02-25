/*
 * Slices let you reference a contiguous sequence of elements in a collection.
 *
 * A slice is a kind of reference, it does not have ownership.
 *
 * Programming question(simple): Write a function that takes a string of words separated by spaces and returns the first word it winfs in that string. If the functions does not find a space in the string, the whole string must be one word, so the entire string should be returned.
*/

fn main() {
    println!("Hello, world!");

    let mut s = String::from("Hello world");

    let fw = first_word(&mut s); // fw will get the value 5

    // s.clear(); // this empties the String, making it equal to ""
    print!("The first word is: {}", fw);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    println!("bytes: {:?}", bytes); // ASCII values of individual letters

    println!("bytes iter: {:?}", bytes.iter());

    println!("bytes iter enumerate: {:?}", bytes.iter().enumerate());

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }
    //

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Tracking a starting and an ending index, have even more values that will be calculated from the data
// fn second_word(s: &String) -> (usize, usize) {}
//
/*
 * String slcies
 *
 * Reference to a contiguous sequence of the elements of a String
 *
 * looks like this:
 * let s = String::from("hello world");
 * let hello = &s[0..5];
 * let world = &s[6..11];
 *
 * Rust's .. range syntax, if you want to start at index 0, drop the value before the two periods.
 *
 * let slice = &s[0..2]
 * let slice = &s[..2] # SAme thing
 *
 *
 * let len = s.len()
 * let slice = &s[3..len]
 * let slice = &s[3..]
 */
