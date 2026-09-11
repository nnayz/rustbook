/*
 * Rust says there are two types of errors
 * Recoverable Errors: File not found (mmost likely just want to report the problem to the user and retry the operation.)
 *
 * Unrecoverable Errors: always symptoms of bugs, index out of range error
 *
 * Rust does not have exceptions.
 * Instead, it has type Result<T, E> for recoverable error
 * and panic! macro that stops execution for unrecoverable error
 *
 *
 * By default when a panic occurs, the programs starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. (Lot of work)
 *
 * We can choose the alternative of immediately aborting.
 *
 * A BACKTRACE is a list of all the functions that have been called to get to this point.
 */

/*
 * Recoverable Errors with Result
 * not serious enough to require program to stop entirely
 *
 * enum Result<T, E> {
 *   Ok(T),
 *   Err(E),
 * }
 */

use std::{fs::File, io::ErrorKind};

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    let greeting_file_result = File::open("hello.txt"); // Return type is Result<T, E>

    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(e) => panic!("Problem opening the file: {e:?}"),
    // };

    /*
     * Matching on different errors
     *
     */

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {e:?}"),
            },
            _ => panic!("Problem opening the file: {e:?}"),
        },
    };
}
