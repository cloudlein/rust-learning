fn main() {
    let mut count = 1;

    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    /*
        False Condition
        The while loop checks the condition before each loop,
        so if the condition is false at the start, the loop will not run at all
    */

    let count = 10;

    while count <= 5 {
        println!("This won't be printed.");
    }

    /*
        Stop a While Loop
        You can stop a while loop when you want by using break
    */

    let mut num1 = 1;

    while num1 <= 10 {
        if num1 == 6 {
            break;
        }
        println!("Number: {}", num1);
        num1 += 1;
    }

    /*
        This prints numbers from 1 to 5 (stops the loop when num reaches 6).
        Next: Learn how to use the for loop to go through a range of values.
    */

    /*
        Skip a Value
        You can skip a value by using the continue statement
    */

    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            num += 1;
            continue;
        }

        println!("Number: {}", num);
        num += 1;
    }

}