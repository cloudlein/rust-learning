fn main(){
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;
    
    println!("BIRTHYEAR: {}", BIRTHYEAR);
    println!("MINUTES_PER_HOUR: {}", MINUTES_PER_HOUR);
    
    /*
        Constants Must Have a Type
        You must write the type when creating a constant. You cannot let Rust guess the type like you can with regular variables

        const BIRTHYEAR: i32 = 1980; // Ok
        const BIRTHYEAR = 1980; // Error: missing type

        Naming Rules
        Another thing about constants, is that it is considered good practice to declare them with uppercase.
        It is not required, but useful for code readability and common for Rust programmers:
        Examples:
         - MAX_SPEED
         - PI
         - MINUTES_PER_HOUR

    */
}