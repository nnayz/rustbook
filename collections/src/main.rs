/*
 * number of very useful data structures called collections.
 *
 *
 * vector: store a variable number of values next to each other.
 * string: collection of characters
 * hashmap: key value
 */

// Importing hashmaps
use std::collections::HashMap;

fn main() {
    // Creating a new vector Vec<T>
    let v: Vec<i32> = Vec::new();

    println!("{v:?}"); // []

    let v = vec![1, 2, 3];

    println!("V: {v:?}");

    // mutable vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // reading
    // via indexing or using the get method
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // this will panic if the index does not exist

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let third: i32 = v[2];

    println!("{third}");

    /*
     * When the program has a valid referemce, the borrow checker enforces the ownership and borrowing rules to ensure that this reference and any other references to the contents of the vector remain valid
     *
     * THIS WONT WORK:
     * let mut v = vec![1, 2, 3, 4, 5];
     * let first = &v[0];
     *
     * v.push(6);
     *
     * println!("The first element is: {first}");
     */

    // This will also not work (mutable references)
    // v.push requires the mutable borrow
    // let mut v = vec![1, 2, 3];

    // let first: &mut i32 = &mut v[0];

    // v.push(6);

    // println!("first element: {first:?}");

    /*
     * Iterating over the values in a vector
     */

    let mut v = vec![100, 32, 57];

    for j in &v {
        // immutable references
        println!("{j}");
    }

    // mutable references iterating
    for j in &mut v {
        *j += 10;
        println!("{j:?}");
    }

    /*
     * Vectors can use enums to hold values of different types but technically they will be o fthe same enum type
     */

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {row:#?}");

    /*
     * Drop
     */

    {
        let _ = vec![1, 2];
    } // vector goes out of scope

    /*
     * Storing UTF-8 Encoded Text with Strings
     *
     *
     * string slices are referenecs to some UTF-8 encoded string data stored elsewhere.
     *
     * String literal are stored in the program's binary and are therefore string slices
     *
     * both String and &str are UTF-8 encoded
     */

    // Creating a string
    let mut s = String::new();

    s = String::from("hello");
    println!("s: {s}");

    let data = "initial contents"; // to_string() is available on any type which implements the Display trait

    let s = data.to_string();

    println!("s: {s}");
    // direct usage on literal.
    let s = "initial contents".to_string();
    println!("s: {s}");

    /*
     * using from
     */

    let s = String::from("initial contents");
    println!("s: {s}");

    /*
     * Strings are UTF-8 encoded, can include any properly encoded data in them.
     */

    let hello = String::from("السلام عليكم");
    println!("hello: {hello}");

    /*
     * We can grow a String by using the push_str method to append a string slice
     */

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";

    s1.push_str(s2); // takes a string slice and does not take ownership
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    let ch = 'l';
    s.push(ch); // Also, does not take ownership

    println!("ch: {ch}");

    /*
     * Concatenating with + or format!
     */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("s3: {s3}");

    /*
     * Signature of +
     * fn add(self, s: &str) -> String {
     * }
     */

    let s4 = String::from("tic");
    let s5 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let res = s4 + "-" + &s2 + "-" + &s3;

    let resformat = format!("{s5}-{s2}-{s3}");

    println!("add operator: {res}\nformat macro: {resformat}");

    /*
     * Indexing into Strings
     */

    let s1 = String::from("hi");
    // let h = s1[0]; Will get an error

    /*
     * A String is a wrapper over Vec<u8>
     */

    let hello = String::from("Hola");

    // In this case, len will be 4 which means the vector storing the string "Hola" is 4 bytes long.

    // let ind = &hello[0];  WIll panic

    /*
     * Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust's perspective; as bytes, scalar values, and grapheme clusters (The closest thing to what we would call letters).
     *
     *
     * Indexing into a string is often a bad idea because it is not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
     */

    let hello = "good morning";
    let s = &hello[0..4];

    println!("s: {s}");

    /*
     * Iterating over strings
     * using chars method
     */

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    /*
     * Hash maps
     *
     * last of the common collections
     * type HashMap<K, V>
     * stores mapping of keys with associated values using hashing function (determines how it places these keys and values into memory)
     *
     * They store the data on the heap
     * All the keys must have same type
     * all the values must have same type
     */

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Blue team score: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /*
     * For types that implement the Copy trait, like i32, the values are copied into hashmap unlike the owned values like String, the values are moved and the hash map will be the owner of those values.
     */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    /*
     * Overwriting a value
     */

    let mut mp = HashMap::new();

    mp.insert(String::from("Blue"), 10);
    mp.insert(String::from("Blue"), 25);

    println!("{mp:?}");

    // Adding a key and value only if the key is not present

    let mut mp = HashMap::new();

    mp.insert(String::from("Blue"), 10);

    mp.entry(String::from("Yellow")).or_insert(50);
    mp.entry(String::from("Blue")).or_insert(50); // Will not update because Blue is already present

    println!("{mp:?}");

    /*
     * Updating the value based on the old value
     */

    let text = "hello world wonderful world!";

    let mut mp = HashMap::new();

    for word in text.split_whitespace() {
        let count = mp.entry(word).or_insert(0); // Returns a mutable reference on `Entry` enum (may or may not exist)
        *count += 1;
    }

    println!("{mp:?}");

    /*
     * Hashmaps uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
     *
     *
     * There is an option to switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher Trait.
     */
}
