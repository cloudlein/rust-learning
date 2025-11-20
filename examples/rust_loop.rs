fn main() {

    let mut count = 0;

    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    /*
        Example explained:

        This prints "Hello World!" 3 times.
        It uses a counter to keep track of how many times it has looped.
        The counter starts at 1 (let mut count = 1;).
        Each time the loop runs, the counter goes up by 1: (count += 1;).
        When it reaches 3, the loop stops.

    */

    let mut count1 = 1;

    let result = loop {
        println!("Hello!");

        if count1 == 3 {
            break count1; // Stop the loop and return the number 3
        }

        count1 += 1;
    };

    println!("The loop stopped at: {}", result);

}