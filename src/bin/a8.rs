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

enum Flavor {
    Lemonade,
    GrapeJuice,
    AppleJuice,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f32,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::AppleJuice => println!("flavor: Apple, juice oz: {:?}", drink.fluid_oz),
        Flavor::Lemonade => println!("flavor: Lemonade, oz: {:?}", drink.fluid_oz),
        Flavor::GrapeJuice => println!("flavor: Grape, juice oz: {:?}", drink.fluid_oz),
    }
}

fn main() {
    let apple_juice: Drink = Drink {
        flavor: Flavor::AppleJuice,
        fluid_oz: 12.0,
    };
    print_drink(apple_juice);

    let lemonade: Drink = Drink {
        flavor: Flavor::Lemonade,
        fluid_oz: 10.0,
    };

    print_drink(lemonade);
}
