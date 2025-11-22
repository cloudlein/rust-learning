fn main() {
    /*
        Structs
        - A struct (short for "structure") is a custom data structure that lets you group related values together.
        - You can think of a struct like a mini-database for one thing, like a person with a name and age.
    */

    /*
        Create a Struct
        You define a struct using the struct keyword and place the fields (variables) inside:
    */

    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    /*
        Once you have a struct, you can create an object of it.
        Then, you can access the fields of the struct using dot syntax (.):
    */

    let mut user = Person{
        name: String::from("John Doe"),
        age: 20,
        can_vote: true,
    };

    // access and print the values
    println!("Name : {}", user.name);
    println!("Age : {}", user.age);
    println!("Can vote: {}", user.can_vote);

    /*
        info: Fields are similar to variables, but they belong to a struct.
        Since they are part of a larger structure (like Person or Car),
        they are called fields in Rust, not regular variables.
    */

    /*
        Change a Field
        To change a value inside a struct, you must make the struct object mutable by using mut:
    */

    user.age = 30;
    println!("Updated Age : {}", user.age);

    /*
        Why Use Structs?
        - To group related data in a clean way
        - To make your code easier to read and maintain
        - To create real-world examples, like users, books, cars, etc.
    */

    
}