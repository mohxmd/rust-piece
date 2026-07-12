// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[allow(dead_code)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct Shoes(Color);
struct Shirt(Color);
struct Pants(Color);

impl Shoes {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Shirt {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Pants {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_color(color: &Color) -> &str {
    match color {
        Color::Black => "black",
        Color::Blue => "blue",
        Color::Brown => "brown",
        Color::Custom(custom) => custom.as_str(),
        Color::Gray => "gray",
        Color::Green => "green",
        Color::Purple => "purple",
        Color::Red => "red",
        Color::White => "white",
        Color::Yellow => "yellow",
    }
}

fn print_shoes(shoes: Shoes) {
    println!("shoes: {}", print_color(&shoes.0));
}

fn print_shirt(shirt: Shirt) {
    println!("shirt: {}", print_color(&shirt.0));
}

fn print_pants(pants: Pants) {
    println!("pants: {}", print_color(&pants.0));
}

fn main() {
    let shoes = Shoes::new(Color::Black);
    let shirt = Shirt::new(Color::Custom("teal".to_string()));
    let pants = Pants::new(Color::Blue);

    print_shoes(shoes);
    print_shirt(shirt);
    print_pants(pants);
}
