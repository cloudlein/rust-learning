fn main() {
    /*
        There are two main types of strings in Rust:
        - &str - is called "string slices", and is used for fixed text like "Hello"
        - String - used when you need a string that can change
        In this chapter, you will mostly work with the String type because it is more flexible and can be changed over time.
    */

    /*
        Create a String
        You can create a String from a string literal using the to_string() method or the String::from() function:
    */

    let text1 = "Hello World".to_string();
    println!("Text 1: {}", text1);

    let text2 = String::from("Hello World");
    println!("Text 2: {}", text2);

    /*
        Change a String
        Strings are mutable, so you can change them if they are declared with mut.
        Use push_str() to add text to a string
    */

    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting); // Hello World

    // Use push() to add one character:
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word); // Hi!
}