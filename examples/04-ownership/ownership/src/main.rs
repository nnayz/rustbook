// Ownership is a set or rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer's memory while running.
//
// Stack and Heap
// In a systems programming language like Rust, whether a value is on the stack or the heap effects how the language behaves and why you have to make certain decisions.
//
//
// Both the Stack and heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. (LIFO)
// All data stored on the stack must have a known, fixed size. Data with an unkown size at compile time or a size that might change must be stored on the heap instead.
//
//
// Heap is less organized. When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that lcoation. This is called allocating on the heap and is sometimes called allocating (pushing values onto the stack is not allocating).
//
// Because the pointer to the heap is a known, fixed size, we store the pointer on the stack, but when you want the actual data, you must follow the pointer.
//
//
// Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
//
// Accessing data on the heap (Pointer following) is generally slower.
//
// When the code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables gget pushed onto the stack. When the function is over, those values are popped off the stack.
//
//
//
//
// Ownership Addresses these problems:
// 1. Keeping track of what parts of code are using what data on the heap
//
// 2. Minimising the amount of duplicate data on the heap
//
// 3. Cleaning up unused data on the heap so that you do not run out of space are all problems that ownership addresses.

// use std::io;

fn main() {
    let rules = String::from(
        "
        The Rules of Ownership\n
        1. Each value in Rust has an owner.\n
        2. There can only be one owner at a time.\n
        3. When the owner goes out of scope, the value will be dropped.\n
    ",
    );

    println!("{}", rules);

    println!("Illustrating Ownership");
    println!(
        "Primitive Data types are of fixed size and can be stored on the stack, we will look into String type which is stored on the heap and see how Rust knows when to clean up that data"
    );

    let mut s = String::from("Hello");

    s.push_str(", world!"); // apends this string literal to a String

    println!("{s}");

    println!("
        With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:\n

        1. The memory must be requested from the memory allocator at runtime.\n
        2. We need a way of returning this memory to the allocator when we’re done with our String.
    ");

    println!("
        In Rust, the memory is automatically returned once the variable that owns it goes out of scope
    ");

    // :: namespace
    //

    // Bind value 5 to x, make a copy of value of x and bind it to y. We have two variables. These have fixed size and stored on the stack.
    // let x = 5;
    // let y = x;
    //
    // Let us look at String version
    //
    // let s1 = String::from("Hello");
    // let s2 = s1;
    //
    // Same thing is not happening here.
    //
    // String is made up of three parts.
    //
    // 1. Pointer to the memory --- holds contents of the string,
    // 2. a length (amount of memory in bytes contents of the String are using.)
    // 3. a capacity (total amount of memory, in bytes, that the String has received from allocator)
    //
    // This group is stored on the stack
    //
    //
    // When s1 is assigned s2, the String data is copied, we copy the pointer, the length, and the capacity that are on the stack. WE DO NOT COPY THE DATA ON THE HEAP that the pointer refers to.
    //
    //
    // AS mentioned earlier, Rust frees up memory using the drop method automatically when the variable goes out of scope. However, in this situation both of the variables use the same memory space in the heap,
    //
    // This will lead to a double free error and is one of the memory safety bugs.
    //
    // To remedy this, after let s2 = 1; Rust considers s1 as no longer valid
    //
    //
    // Check:
    // let s1 = String::from("Hello");

    // let s2 = s1; // s2 points to the same mem location

    // This looks similar to shallow copy, but is in fact called a move since s1 is no longer valid.

    let s1 = String::from("Hello");
    println!("{s1}, world!");

    // Rust never makes deep copies of the data automatically.
    //
    // The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well. When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately. Consider this code, for example:
    //
    let mut s = String::from("Hello!");
    s = String::from("ahoy"); // free called and mem space is freed and then ahoy's ptr, len, cap is pushed onto the stack and the ahoy is being allocated on the heap.

    println!("{s}, world;");

    println!("===================\nDeep copies\n\n");

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack. Variables that use it do not move but are trivially copied.
    //
    // Rust will not let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
    //
    //
    // Passing a value to a function are similar to those when assigning a value
    //
    // Move or copy just as assignment does.
    //
    demonstration();

    // Returning values can also transfer ownership.
    //
    demonstration_2();

    // The ownership of a variable follows the same pattern every time. Assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.
    //
    //
    // Returning multiple values

    demonstration_3();
}

fn demonstration_2() -> () {
    let s1 = gives_ownership(); // gives_ownership moves its return

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 moved into takes_and_gives_back, which also moves its return value into s3
}

fn gives_ownership() -> String {
    print!("[gives_ownership]");
    let some_string = String::from("yours"); // comes into scope

    some_string // moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    print!("[takes_and_gives_back]");

    a_string // returned and moves out to the calling function
}

fn demonstration() -> () {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...

    let x = 5; // x comes into scppe

    makes_copy(x); // Because i32 implements the Copy trait, x foes NOT move into the function, it is okay to use x afterward.
}

fn takes_ownership(some_string: String) -> () {
    println!("[takes_ownership] {some_string}");
} // some_string go out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) -> () {
    println!("[makes_copy] {some_integer}");
}

fn demonstration_3() -> () {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("THe length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of the String

    (s, length)
}
