fn main() {

    /*
        Rust has the following conditional statements:
        Use if to specify a block of code to be executed, if a specified condition is true
        Use else to specify a block of code to be executed, if the same condition is false
        Use else if to specify a new condition to test, if the first condition is false
        Use switch to specify many alternative blocks of code to be executed
    */

    // if
    if 7 > 5 {
        println!("The number greater than 5");
    }

    // use variable
    let x = 7;
    let y = 8;

    if x > y {
        println!("The number greater than y");
    }

    // if .. else
    let age = 16;

    if age >= 18 {
        println!("The number greater than 18");
    }else {
        println!("The number less than 18");
    }

    // else if

    let score = 85;
    if score >= 90 {
        println!("Grade A")
    }else if score >= 80 {
        println!("Grade B")
    }else if score >= 70 {
        println!("Grade C")
    }else if score >= 60 {
        println!("Grade D")
    }else {
        println!("Grade E")
    }

    // using if as an expression
    /*
        In Rust, if...else can also be used as an expression.
        This means you can assign the result of an if to a variable
    */

    let time = 18;
    let greeting = if time < 18 {
        "good morning"
    }else {
        "good evening"
    };

    println!("The greeting is {}", greeting);

    // Simplified Syntax

    let time1 = 20;
    let greeting1 = if time < 24 { "good morning" } else { "good evening" };
    println!("The greeting is {}", greeting1);

    /*
        Tip: This works similarly to the ternary operator (condition ? value1 : value2)
        in languages like Java or C. However, Rust does not have a ternary operator,
        but using if...else as an expression gives you the same effect.
    */

    /*
        Note: The value from if and else must be the same type, like two pieces of text or two numbers
        (in the example above, both are strings).
        When you mix types, like a string and an integer, you'll get an error:
    */

    /*
        Example
        let number = 5;
        let result = if number < 10 { "Too small" } else { 100 };
        println!("{}", result);
        Result: error[E0308]: `if` and `else` have incompatible types
    */

}