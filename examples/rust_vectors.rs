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
    println!("{:?}", fruits); // ["apple", "banana", ,"orange"]

    /*
        Add or Remove Elements at a Specified Index
        Rust vectors are designed to grow and shrink at the end, but you can also add or remove elements at the beginning or at a specified index.
        Use insert() to add an item at a specified index:
    */

    fruits.insert(1, "grape");
    println!("{:?}", fruits); //  // ["apple", "grape" ,"banana", "orange"]

    /*
        Remove the First Item
        Use remove() to remove an element from a specified index:
    */

    // remove banana
    fruits.remove(2);
    println!("{:?}", fruits); // ["apple", "grape" , "orange"]

    // Note: Adding or removing elements from the beginning is slower than at the end, because all the other elements have to shift positions.

    /*
        Vector Length
        You can find out how many elements there are in a vector using the .len() method:
    */

    println!("there are {} fruits", fruits.len());

    /*
        Loop Through a Vector
        Just like arrays, you can use a for loop to go through all the values in a vector:
    */

    for fruit in &fruits {
        println!("{}", fruit);
    }

    /*
        Note: Use &fruits to borrow the vector instead of moving it.
        In Rust, borrowing means using a reference to a value instead of taking ownership of it.
        When you loop through a vector without &, the values are moved out, and you can no longer use the vector.
        But when you borrow the vector using &, you can still use it later in your program.
    */
}