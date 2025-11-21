use std::collections::HashMap;
use std::ptr::hash;

fn main() {
    /*
        In Rust, data structures are used to store and organize values.
        Rust provides many built-in data structures. Each is used to handle data in different ways.
        Some of the most common are:

        - Array
        - Vector (Vec)
        - Tuple
        - HashMap

    */

    /*
        Arrays
        An array in Rust is a fixed-size list of values, all of the same type.
        You cannot grow or shrink an array after it's created.
        To access an array element, refer to its index number.
        Array indexes start with 0: [0] is the first element, [1] is the second element, etc.
    */

    let numbers = [1, 2, 3];
    println!("{:?}", numbers);
    println!("Last number: {}", numbers[2]);

    /*
        Vectors
        A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.
    */

    let mut fruits = vec!["apple", "pinapple", "melon"];
    fruits.push("grape");
    println!("fruits: {:?}", fruits);
    println!("last fruit is : {}", fruits[fruits.len() - 1]); // if we dont know what index
    println!("last fruit is : {}", fruits[3]);

    /*
        Tuples
        A tuple can hold multiple values of different types.
        It is useful when grouping different types together.
        You access tuple elements using a dot and an index number, like person.1, etc
    */

    let person = ("John Doe", 30, true);
    println!("person: {:?}", person); // print all
    println!("Name : {}", person.0);
    println!("Age : {}", person.1);
    println!("is active : {}", person.2);

    /*
        HashMaps
        A HashMap stores key-value pairs. It lets you look up a value using a key.
        To use HashMap, you must import it from the standard library.

    */

    let mut capitalCities = HashMap::new();
    capitalCities.insert("France", "Paris");
    capitalCities.insert("Japan", "Tokyo");
    println!("Capital Cities: {:?}", capitalCities);

    println!("Capital Cities: {:?}", capitalCities.get("France"));
    println!("Capital of Japan is {}", capitalCities["Japan"]);


}