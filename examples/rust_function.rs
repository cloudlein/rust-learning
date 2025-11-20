fn main() {
    /*
        Creating a Function
        To create a function, use the fn keyword, followed by the function name and a set of parentheses () and curly braces {}:
        Example:
        fn function_name() {
          // code to be executed
        }

        Calling a Function
        Now that you have created a function, you can execute it by calling it.
        To call a function, write the name of the function followed by two parantheses ().

        Example
        // Create a function
        fn say_hello() {
          println!("Hello from a function!");
        }

        say_hello(); // Call the function
   */

    fn greeting(name: &str) {
        println!("Hello, good morning {}!", name);
    }

    greeting("Rust");

     /*
        Functions with Parameters
        You can send information into a function using parameters. Parameters are written inside the parentheses ().
    */

    fn plus_one(x: i32) {
        println!("x is: {}", x);
    }

    plus_one(1);

    /*
        Functions with Return Values
        A function can also return a value.

        Use the  -> symbol in the function header to show what type of value will be returned.
        Inside the function, use the return keyword to send the value back
    */

    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }

    let sum = add(3, 4);
    println!("Sum is: {}", sum);

    /*
        In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:
    */

    fn add_one(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add_one(3, 4);
    println!("Sum is: {}", sum);

}