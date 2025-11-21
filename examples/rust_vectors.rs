fn main() {
    /*
        Vectors
        A vector is a resizable array. Unlike regular arrays, vectors can grow or shrink in size.
    */

    /*
        Creating a Vector
        To create a vector, use the vec! macro:
    */

    let mut fruits = vec!["apple", "banana", "orange"];

    /*
        Access Vector Elements
        You can access values in a vector using index numbers (just like arrays)
    */

    println!("First fruit: {}", fruits[0]);

    /*
        Change Vector Values
        To change a value in the vector, refer to the index number and assign a new value.
        Remember to make the vector mutable (using the mut keyword):
    */

    fruits[0] = "grape";
    println!("New first fruit: {}", fruits[0]);

    /*
        Add Elements to a Vector
        You can add a new element to the end of a vector using the push() method:
    */

    fruits.push("cherry");
    println!("{:?}", fruits); // ["apple", "banana", ,"orange", cherry"]

    /*
        Remove Elements from a Vector
        To remove the last element from a vector, use pop():
    */

    fruits.pop();
    println!("{:?}", fruits);

    /*
        Add or Remove Elements at a Specified Index
        Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.
        Use insert() to add an item at a specified index:
    */

}