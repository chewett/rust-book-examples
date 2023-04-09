fn main() {
    let mut value = 4;
    println!("The value of the variable is {value}");
    value = 5;
    println!("The value of the variable is {value}");

    {
        let value = 6; //Shadowing the previous variable
        println!("The value of the variable is {value}");
    }

    // But the shadow is only in the scope above
    println!("The value of the variable is {value}");

    const TIME_IN_HOURS: u32 = 60 * 60;
    println!("The number of seconds in an hour is {TIME_IN_HOURS}")


}
