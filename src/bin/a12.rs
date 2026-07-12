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

// * Use an enum for the box color

#[allow(dead_code)]
enum Color {
    Green,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Green => println!("Color: Green"),
            Color::Blue => println!("Color: Blue"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        let (width, height, depth) = (self.width, self.height, self.depth);
        println!("Width: {:?}", width);
        println!("Height: {:?}", height);
        println!("Depth: {:?}", depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        depth: 3.0,
        width: 2.0,
        height: 2.0,
    };

    let free_box = ShippingBox::new(5.0, Color::Green, small_dimensions);
    free_box.print();
}
