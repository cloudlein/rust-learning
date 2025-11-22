fn main() {
    /*
        Enums
        An enum (short for "enumeration") is a way to define a type that can be one of a few different values.
        Each value in the enum is called a variant.
        Enums are useful when you want to represent a value that can only be one of a set of options - like days of the week,
        directions, or results like success and error.
    */

   // To use the enum, create a variable and assign it one of the enum's variants (use :: to access a variant):

    let my_direction = Direction::Up;
    println!("We are going to up");

    /*
        Match on Enum Values
        Enums work great with the match statement. You can run different code depending on which variant is used:
    */

    match my_direction {
        Direction::Up => {
            println!("We are going to up");
        }
        Direction::Down => {
            println!("We are going to down");
        }
        Direction::Left => {
            println!("We are going to left");
        }
        Direction::Right => {
            println!("We are going to right");
        }
    }

    let result_login = LoginStatus::Success(String::from("Welcome John!"));

    match result_login {
        LoginStatus::Success(message) => {
            println!("Success : {}", message);
        }
        LoginStatus::Error(message) => {
            println!("Error: {}", message);
        }
    }

    /*
        Why Use Enums?
        - To group related values into one type
        - To make your code more readable and safe
        - To handle different cases with match
    */

}

/*
   Create an Enum
   To create an enum, use the enum keyword and add a set of named values (variants) separated by commas:
*/
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/*
    Enums with Data
    Enum variants can also hold data. This is useful when each variant needs to store extra information:
*/

enum LoginStatus {
    Success(String),
    Error(String),
}
