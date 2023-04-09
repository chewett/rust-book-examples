fn main() {
    // parse needs typehinting so it knows what format to parse it to.
    // Again parse returns Result so we handle the Err enum result with expect() to crash if it fails
    let guess: u32 = "42".parse().expect("Not a number!");

    // _ can be used as a number separator
    let aaa : u128 = 1_000_000_000;

    //Tuples are defined with ( )
    let tup = (true, 'W', 123123213, String::from("A string of some sorts"));
    let tup2 : (bool, char, i128, String) = (true, 'W', 123123213, String::from("A string of some sorts"));

    //And they can be unpacked with let and brackets
    let (w, x, y, z) = tup;
    println!("W is set to {w}");

    //Arrays in rust have a fixed length
    let array = [1, 2, 3, 4, 5];
    // And can be typehinted
    let array : [i64; 3] = [1, 3, 4];



}
