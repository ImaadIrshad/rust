// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

fn main() {
    enum Colors {
        Red,
        Green,
        Blue
    }
    fn print_color(color: Colors) {
        match color {
            Colors::Blue => println!("Blue"),
            Colors::Red => println!("red"),
            Colors::Green => println!("Green"),
        }
    };
    print_color(Colors::Red);
    practice(AnotherColor::Red);
    practiceagain(Anotheranothercolor::Seagreen)
}

enum AnotherColor {
    Green,
    White,
    Red
}

fn practice(argument: AnotherColor) {
    match argument {
        AnotherColor::Green => println!("guess what its Green"),
        AnotherColor::White => println!("guess what its White"),
        AnotherColor::Red => println!("guess what its Red")
    }
}

enum Anotheranothercolor {
    Yellow,
    Turquoise,
    Seagreen
}

fn practiceagain(color: Anotheranothercolor) {
    match color {
        Anotheranothercolor::Yellow => println!("it's a yellow"),
        Anotheranothercolor::Turquoise => println!("it's a Turquoise"),
        Anotheranothercolor::Seagreen => println!("it's a Seagreen"),
    }
}