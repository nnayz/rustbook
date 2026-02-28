/*
 * Enumerations: enums
 *
 * define a type by enumerating its possible variants
 *
 * exampele Ip addresses
 */

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

/*
 * Define a function that takes any IpAddrKind
 */

// fn route(ip_kind: IpAddrKind) {}

// Can also do this

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/*
 * However, we can also do this, rather than an enum inside a stuct , we can put data directly into each enum variant.
 */

#[derive(Debug)]
enum IpAddrDataEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

/*
 *
 * Another example
 */

enum Message {
    Quit,                       // No data associated
    Move { x: i32, y: i32 },    // has named fields
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// impl block for enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/*
 * Option Enum
 *
 * encodes the very common scenario in which a value could be something or it could be nothing
 *
 * VERY USEFUL
 *
 * Eliminating the risk of incorrectly assuming a not-null value helps you be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
 */

// enum Option<T> {
//     None,
//     Some(T),
// }

/*
 * The match Control Flow Construct
 *
 * Execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things.
 */

/*
 * The Option<T> match Pattern
 *
 * using match
 */

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // Rest of the states
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

/*
 * Catch all patterns and the _ placeholder
 *
 * specific behaviour on some specific cases, and default behaviour for all other possible cases
 */

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("THe home IP address: {:#?}", home.address);
    println!("The kind of loopback IP address: {:#?}", loopback.kind);

    // Instantiating those data enums
    let home2 = IpAddrDataEnum::V4(127, 0, 0, 1);

    let loopback2 = IpAddrDataEnum::V6(String::from("::1"));

    println!("The home2 data enum: {:#?}", home2);
    println!("The loopback2 data enum: {:#?}", loopback2);

    let m = Message::Write(String::from("hello"));
    m.call();

    // Instantiating Option Enum
    // let some_number = Some(5); // Option<i32>
    // let some_char = Some('e'); // Option<char>

    // let absent_number: Option<i32> = None;

    let state = UsState::Alaska;
    let coin = Coin::Quarter(state);

    let cents = value_in_cents(coin);

    println!("Cents: {cents}");

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    println!("none: {none:?}, six: {six:?}");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    /*
     * Rust also has a pattern we can use when we want a catch-all but do not want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value. This tells Rust we are not going to use the value
     */

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    /*
     * if let and let...else
     *
     * if let lets you combine if and let
     * Handle values that match one pattern while ignoring the rest.
     */

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => {}
    }

    /*
     * Shorter way
     */
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}")
    }

    /*
     * Example
     */
    let state2 = UsState::Alabama;
    println!("The state: {:?}", state2.existed_in(1900));

    let coin = Coin::Quarter(state2);

    describe_state_quarter(coin);

    /*
     * Rust also has a let...else syntax
     */

    fn describe_state_quarter_2(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }

    let state = UsState::Alabama;
    let coin = Coin::Quarter(state);

    // Usage
    describe_state_quarter_2(coin);
}
