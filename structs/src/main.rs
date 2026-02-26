/*
 * Struct or structure is a custom data type that lets you package together and name multiple related values that make upa  meaningful group
 */

#[derive(Debug)] // Automatically implements the Debug Trait for the struct which allows it to be printed.
struct User {
    active: bool, // These are called fields
    username: String,
    email: String,
    sign_in_count: u64,
}

// Creating different types with Tuple Structs
// They do not have names associated to their fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Defining Uni-like Structs
// They do not have any fields
// Useful when need to implement a trait on some type but do not have any data that you want to store  in type itself.
#[derive(Debug)]
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field name and var name is name
        email,
        sign_in_count: 1,
    }
}

// There is a struct update syntax too, if you want to change some of the types.

fn main() {
    // Defining and Instantiating Structs

    let user: User = User {
        active: true,
        username: String::from("username"),
        email: String::from("user@example.com"),
        sign_in_count: 1,
    };

    // user.email = String::from("changed_user@example.com");

    let user2: User = build_user(String::from("user2@example.com"), String::from("user2"));

    println!("User2: {:?}", user2);

    // Struct update syntax and use the sprad operator
    // Uses assignment operator and the data will be moved
    //
    // We can no longer use `user` as the String value  field was moved to user3, if we had given new String values for both email and username, then we couldve used it.
    //
    // Both active and sign in count are types that implement the Copy trait.
    let user3: User = User {
        email: String::from("anotherUser@example.com"),
        ..user
    };

    println!("{:#?}", user.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Destructuring these tuple struct instances

    let Point(x, y, z) = origin;
    println!("{} {} {}", x, y, z);

    // Instantiating the unit like struct
    let subject = AlwaysEqual;

    println!("{:#?}", subject); // Prints AlwaysEqual

    // Ownership of Struct Data
    // used owned String type rather than the &str string slice type. deliberate choice becuase we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid

    let width = 30;
    let height = 50;

    println!(
        "[Naive] The area of the rectangle is {} square pixels",
        area(width, height)
    );

    // Rectoring with tuples
    let rect = (30, 50);

    println!(
        "[Tuples] The area of the rectangle is {} square pixels",
        area_with_tuples(rect)
    );

    // Reactoring with structs
    // Types are outside main due to scoep contraint
    //
    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "[Structs] The area of the rectangle is {:#?} square pixels",
        area_with_structs(&rect2) // Does not take ownership
    );

    println!("[Structs] The rectangle is : {:#?}", rect2);

    // Another way to print out a value using the Debug format is to use the dbg! macro which takes ownership of an expression(opposed to println! which takes reference),
    //
    // prints value and the returns ownership

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuples(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_structs(rect2: &Rectangle) -> u32 {
    rect2.width * rect2.height
}
