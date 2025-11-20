fn main() {
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    /*
        Example explained:
        - The match variable (day) is evaluated once.
        - The value of the day variable is compared with the values of each "branch"
        - Each branch starts with a value, followed by => and a result
        - If there is a match, the associated block of code is executed
        - _ is used to specify some code to run if there is no match (like default in other languages).
        - In the example above, the value of day is 4, meaning "Thursday" will be printed
    */

    // Multiple Matches
    // You can match multiple values at once using the | operator (OR):
    let day1 = 6;

    match day1 {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    /*
        match with a Return Value
        Just like if, match can also return a value:
        This means you can save the result of a match into a variable
    */

    let day2 = 4;

    let result = match day2 {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);
}