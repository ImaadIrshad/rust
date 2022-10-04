// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let variable = 42;
    match variable {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("some other number"),
    }
    practice();
    practicewithenum(Numbers::Three)
}

fn practice() {
    let expression1 = 54;
    match expression1 {
        34 => println!("its 34"),
        46 => println!("its 46"),
        54 => println!("its 54 !!!!"),
        _ => println!("its something else")

    }
}

enum Numbers {
    One,
    Two,
    Three,
    Four
}

fn practicewithenum(number: Numbers) {
    match number {
        Numbers::One => println!("Okay its one"),
        Numbers::Two=> println!("Okay its two"),
        Numbers::Three=> println!("Okay its three"),
        Numbers::Four=> println!("Okay its four"),
        _ => println!("Okay its somethine else"),
    }
}
