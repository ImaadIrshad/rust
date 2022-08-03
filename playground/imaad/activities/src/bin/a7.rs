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
        red,
        green,
        blue
    }
    fn print_color(color: Colors) {
        match color {
            Colors::blue => println!("blue"),
            Colors::red => println!("red"),
            Colors::green => println!("green"),
        }
    }
    print_color(Colors::red)
}
