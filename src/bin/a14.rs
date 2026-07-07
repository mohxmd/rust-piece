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

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    fav_color: String,
}

// * The name and colors should be printed using a function
fn print_person(name: &str, fav_color: &str) {
    println!("Name: {:?}", name);
    println!("Color: {:?}", fav_color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let pirates = vec![
        Person {
            age: 9,
            name: "Shanks".to_owned(),
            fav_color: "red".to_owned(),
        },
        Person {
            age: 17,
            name: String::from("Luffy"),
            fav_color: String::from("red"),
        },
        Person {
            age: 19,
            name: String::from("Zoro"),
            fav_color: String::from("green"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for pirate in pirates {
        // * Use an if expression to determine which person's info should be printed
        if pirate.age <= 10 {
            print_person(&pirate.name, &pirate.fav_color);
        }
    }
}
