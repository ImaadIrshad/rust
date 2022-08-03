// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


struct Drink {
    flavor: flavors,
    ounce: f64
}
enum flavors {
    Sweet,
    Spicy
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        flavors::Sweet => println!("Sweet"),
        flavors::Spicy => println!("Spicy")
    }
    match drink.ounce {
        10.0 => println!("Nice tenner"),
        _ => println!("You can go for that aswell.")
    }
    println!("The oz is {:?}", drink.ounce)
}
fn main() {
    let spicy = Drink {
        flavor: flavors::Spicy,
        ounce: 10.0
    };
    print_drink(spicy);
    let sweet = Drink{
        flavor: flavors::Sweet,
        ounce: 15.3
    };
    print_drink(sweet)
}
