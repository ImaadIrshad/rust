// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn main() {
    struct Persons {
        Age: i32,
        Name: String,
        Color: String
    }

    let data = vec![
        Persons {
            Age: 10,
            Name: "Ralph".to_owned(),
            Color: "Red".to_owned() 
        },
        Persons {
            Age: 24,
            Name: "Imaad".to_owned(),
            Color: "Black".to_owned() 
        },
        Persons {
            Age: 9,
            Name: String::from("Ivan"),
            Color: String::from("Blue")
        },
    ];

    fn print_name(name: &str) {
        println!("Name is {:?}", name)
    }

    fn print_color(color: &str) {
        println!("Color is {:?}", color)
    }

    for data in data {
        if data.Age <= 10 {
            print_name(&data.Name);
            print_color(&data.Color);
        }
        
    }


    
}
