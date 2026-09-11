/*
 * Methods are similar to functions.
 * Declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that is run when the method is called somewhere else.
 *
 * However, unlike functions, methods are defined within the context of a struct (or an enum or a trait object)
 *
 * Their first parameter is always self, which represents the instance of the struct the method is being called on.
 *
 *
 * Good for organizing code
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) block for Rectangle
//
// methods must have a parameter self (alias for struct instance of which this is part of)
//
//
// Also, a struct can have more than one impl blocks
impl Rectangle {
    fn area(self: &Self) -> u32 {
        // Did not take ownership
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // Getter function
    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //  we can define associated functions that do not have self as their first param(are not methods) but are associated functions.
    //
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Note: We can choose to give a method the same name as one of the struct's fields. For eg, we can define a method on Rectangle that is also named width

fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels", rect.area());

    println!("The rect is {:#?}", rect);

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }

    println!(
        "The height of the rectangle using the getter method: {:#?}",
        rect.height()
    );

    let smaller_rect: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    let bigger_rect: Rectangle = Rectangle {
        width: 40,
        height: 50,
    };

    println!(
        "Can the bigger_rect can hold the smaller_rect ?\n Answer: {}",
        bigger_rect.can_hold(&smaller_rect)
    );

    println!(
        "Can the smaller_rect can hold the bigger_rect ?\n Answer: {}",
        smaller_rect.can_hold(&bigger_rect)
    );

    // Instantiation of non method associated function (use the :: syntax)
    let square: Rectangle = Rectangle::square(20);
}
