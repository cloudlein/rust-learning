fn main() {

    /*
        What is {}?
        Rust uses {} as a placeholder in println!() to show variable values.

        Using Placeholders in Order
        When you use many placeholders, the values you pass are used in the same order.

        In the example above:

        The first {} gets replaced with name ("John")
        The second {} gets replaced with age (30)
        Important: The order matters. If you switch the values, the output will change:

    */

    let name = "John Doe";
    let age = 20;

    println!("My name is {} and I am {}", name, age);

    /*
        Variable Values Cannot be Changed by Default
        By default, variables in Rust cannot be changed after they are created:
    */

    // let x = 5;
    // x = 10; // error
    // println!("x is {}", x);

    /*
        Change Variable Values
        If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable)
    */

    let mut x = 5;
    println!("before: {}", x);
    x = 10;
    println!("after: {}", x);

}