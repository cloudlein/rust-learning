fn main() {
    for i in 0..10 {
        println!("i is {}", i);
    }

    /*
        Inclusive Range
        If you want to include the last number, use ..= (two dots and an equals sign):
    */

    for n in 1..=6 {
        println!("n is {}", n);
    }

    /*
        Break and Continue
        Just like other loops, you can use break to stop the loop and continue to skip a value:
    */

    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }

    /*
        Rust Loops Summary
        Rust has three types of loops that let you run code over and over again.
        Each one is used in different situations:
    */

    /*
        1 loop
          The simplest kind of loop. It runs forever unless you stop it with break.
            loop {
              // do something
              if condition {
                break;
              }
            }

         Use loop when you don't know in advance how many times to repeat.
    */

    /*
        2. while
           Repeats code while a condition is true. It checks the condition before each loop
           while count <= 5 {
              println!("{}", count);
              count += 1;
           }
           Use while when you want to repeat code until something happens.
    */

    /*
        3. for
           Repeats code a fixed number of times.
           for i in 1..=5 {
              println!("{}", i);
           }
           Use for when you know exactly what to loop through.
    */

}