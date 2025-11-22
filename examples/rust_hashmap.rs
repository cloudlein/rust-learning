use std::collections::HashMap;

fn main() {
    /*
        A HashMap is a collection of key/value pairs.

        HashMaps are great when you want to store values and find them by a key.

        To use HashMap, you must import it from Rust's standard library:
        use std::collections::HashMap;
    */

    /*
        Create a HashMap
        You can create a new, empty HashMap and add items to it:
    */

    // create hashmap
    let mut capitalCities = HashMap::new();

    // add keys and values (country, city)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Indonesia", "Jakarta");

    println!("CapitalCities : {:?}", capitalCities);


    /*
        Access Values
        You can use the .get() method to access a value in a HashMap by its key:
    */

    if let Some(city) = capitalCities.get("Indonesia") {
        println!("The capital of indonesia is {}", city);
    }else {
        println!("Capital of indonesia is unknown");
    }

    /*
        Update Values
        If you insert a new value using a key that already exists, the old value is replaced with the new one:
    */

    capitalCities.insert("Indonesia", "Kalimantan");
    println!("CapitalCities : {:?}", capitalCities);

    /*
        Remove Values
        To remove a key from a HashMap, use the .remove() method:
    */

    capitalCities.remove("Indonesia");
    println!("CapitalCities : {:?}", capitalCities);

    /*
        Loop Through a HashMap
        You can use a for loop to go through all key/value pairs:
    */

    for (country, city) in &capitalCities {
        println!("the capital of {} is {}", country, city);
    }

    /*
        Why Use HashMaps?
        - To store data by key
        - To quickly look up values
        - To group related data (like names and scores)
        Note: HashMaps require keys to be unique. Inserting the same key again will overwrite the old value.
    */
}