fn main() {
    /*
        Tuples
        A tuple is a group of values of different types, stored in a single variable.
        Tuples are useful when you want to return or work with multiple values together.
    */

    /*
        Create a Tuple
        Tuples are written using parentheses (), with values separated by commas:
    */

    let person = ("John Doe", 30, true); // This tuple contains a &str, an i32, and a bool.

    /*
        Access Tuple Values
        You can access tuple values by using a dot . followed by the index:
    */

    println!("Name : {}", person.0);
    println!("Age : {}", person.1);
    println!("Is Active: {}", person.2);

    /*
        Unpack a Tuple
        When we create a tuple, we normally assign values to it. This is called "packing" a tuple:
    */

    let (name, age, is_active) = person;
    println!("Name : {}", name);
    println!("Age : {}", age);
    println!("Is Active: {}", is_active);

    /*
        Return a Tuple from a Function
        Tuples are often used to return multiple values from a function
    */

    let user = get_user();
    println!("Name : {}", user.0);
    println!("Age : {}", user.1);

}

fn get_user() -> (String, i32){
    (String::from("Alex Doe"), 30)
}