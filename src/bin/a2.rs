// Basic arithmetic
//
// Program requirements
// * Display the results of the sum of two numbers
//
// Note:
// * Use a function to add two numbers together
// * Use  a function to display teh result
// * Use teh "{:?}" token to println macro to display the result

// * Use a function to add two numbers together
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
// * Use  a function to display teh result
fn display_result(result: i32) {
    // * Use teh "{:?}" token to println macro to display the result
    println!("{:?}", result);
}

fn main() {
    let result: i32 = sum(10, 20);
    display_result(result);
}
