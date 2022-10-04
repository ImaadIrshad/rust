// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


// * Use a function to add two numbers together

fn add(a: i32, b:i32) -> i32{
    a + b
}

fn sub(a: i32, b:i32) -> i32 {
    a - b
}

fn mul(a: i32, b:i32) -> i32 {
    a * b
}

fn div(a:i32, b:i32) -> i32 {
    a / b
}

fn remainder(a: i32, b:i32) -> i32 {
    a % b 
}

// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn display_result() {
    let addition = add(24, 23);
    println!("{:?}" , addition);
    let subtraction = sub(23,12);
    println!("{:?}", subtraction);
    let multiplication = mul(23,3);
    let division = div(32,4);
    println!("{:?} {:?}", multiplication, division);
    let remaindervalue = remainder(1731412312,3);
    println!("{:?}", remaindervalue);
}

fn main() {
    display_result()
}
