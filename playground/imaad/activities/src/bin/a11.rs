// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity is {:?}", item.quantity)
}

fn display_id(item: &GroceryItem) {
    println!("id is {:?}", item.id)
}

fn main() {
  let sub = GroceryItem {
    quantity: 342,
    id: 15,
  };
  display_quantity(&sub);
  display_id(&sub)
}
