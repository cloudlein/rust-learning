fn main() {
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string

    // However, it is possible to explicitly tell Rust what type a value should be:
    /*
        let my_num: i32 = 5;          // integer
        let my_double: f64 = 5.99;    // float
        let my_letter: char = 'D';    // character
        let my_bool: bool = true;     // boolean
        let my_text: &str = "Hello";  // string
    */

    /*
        Basic data types in Rust are divided into different groups:

        Numbers - Whole numbers and decimal numbers (i32, f64)
        Number types are divided into two groups: integer types and floating point types.
            let age:i32 = 25 // integer
            let price:f64 = 19.99 // float

        Characters - Single letters or symbols (char)
            let myGrade: char = 'B'

        Strings - Text, a sequence of characters (&str)
            let name:&str = "john"

        Booleans - True or false values (bool)
            let is_active: bool = true
    */

    // Combining Data Types
    let name = "John";
    let age = 28;
    let is_admin = false;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Admin: {}", is_admin);


}