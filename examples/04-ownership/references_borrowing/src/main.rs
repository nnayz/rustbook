fn main() {
    // A reference is like a pointer in that its an address we can follow to access the data stored at that address.
    //
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Does not take ownership so using s1 after this line is still valid

    println!("The length '{s1}' is {len}.");

    // What happens if we try to modify something we are borrowing. Doesn't work

    // change(&s1);

    // For this to work we need to pass a mutable reference

    let mut s2 = String::from("Hello");

    change(&mut s2);

    // Note: If you have mutable reference to a value, you can have no other reference to that value.
    //
    // ThIS WILL FAIL
    //
    // This prevents data race
    //
    // Data race: similar to a race condition and happens when these three behaviours occur:
    // 1. Two or more pointers access the same data at the same time
    // 2. At least one of the pointers is being used to write to the data
    // 3. There is no mechanism being used to synchronise access to the data

    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("r2: {r2}");

    // THIS WILL WORK
    //
    {
        let r4 = &mut s;
        println!("{r4}");
    } // r4 goes out of scope
    let r3 = &mut s;

    println!("{r3}");

    // Dangling References
    // dangling pointer a pointer that refers to a location in memory that may have been given to someone else, by freeing up some memory while preserving a pointer to that memory.
    //

    // let reference_to_nothing = dangle(); // Illustration
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // throws error: contains borrowed value
//     // The value will be freed after it goes out of scope
// }

// Note: The opposite of reference by using  & is dereferencing, which is acomplished with the dereference operator, *.

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // s goes out of scope, s does not have ownership of what it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    // println!(
    //     "some_string: {some_string} | *some_string:
    //     {:?}",
    //     *some_string
    // );
    some_string.push_str(", world!");
}
