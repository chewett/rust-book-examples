fn main() {
    println!("Hello, world!");
    run_me(123, 'X');
    println!("Five is {}", five());
}

fn run_me(x: i32, y: char) {
    println!("Run me called with values {x} and {y}");
}

//This function returns five as the final expression evaluates to five
fn five() -> i32 {
    4 + 1 // Evaluates to five
}