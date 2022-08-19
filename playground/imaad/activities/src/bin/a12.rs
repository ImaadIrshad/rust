// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    dimensions: f64,
    weight: f64,
    colour: Colour
}

enum Colour {
    red,
    black,
    blue
}

impl Colour {
    fn print(&self) {
        match self {
            Colour::red => println!("Color is red"),
            Colour::black => println!("Color is black"),
            Colour::blue => println!("Color is blue"),
        }
    }
}

impl ShippingBox {
    fn new(dimensions: f64, weight: f64, colour: Colour) -> Self {
        Self {
            dimensions,
            weight,
            colour
        }
    }

    fn show_box(&self) {
        println!("Box details are {:?} {:?}", self.dimensions, self.weight);
        self.colour.print();
    } 
}

fn main() {
    let small_box = ShippingBox::new(32.0, 12.4, Colour::black);
    small_box.show_box();
}
