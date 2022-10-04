// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let value = false;
    match value {
        true => println!("it's true"),
        false => println!("it's false")
    }
    practice()
}

fn practice() {
    let value1 = true;
    match value1 {
        true => println!("its damn true"),
        false => println!("its damn lie"),
        _ => println!("i dont know what this is")
    }
}
