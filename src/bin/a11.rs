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
    id: i32,
    qt: i32,
}

fn display_qt(item: &GroceryItem) {
    println!("Qt: {:?}", item.qt);
}

fn display_id(item: &GroceryItem) {
    println!("Id: {:?}", item.id);
}

fn main() {
    let item = GroceryItem { id: 123, qt: 20 };

    display_id(&item);
    display_qt(&item);
}
