// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
#[allow(dead_code)]
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

#[test]
fn clamp_inside_range() {
    assert_eq!(clamp(5, 1, 10), 5);
}

#[test]
fn clamp_below_range() {
    assert_eq!(clamp(0, 1, 10), 1);
}

#[test]
fn clamp_above_range() {
    assert_eq!(clamp(11, 1, 10), 10);
}

/// Divides a and b.
#[allow(dead_code)]
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[test]
fn div_valid_numbers() {
    assert_eq!(div(10, 2), Some(5));
}

#[test]
fn div_by_zero() {
    assert_eq!(div(10, 0), None);
}

/// Takes two strings and places them immediately one after another.
#[allow(dead_code)]
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

#[test]
fn concat_two_words() {
    assert_eq!(concat("hello", "world"), "helloworld");
}

#[test]
fn concat_with_empty_string() {
    assert_eq!(concat("hello", ""), "hello");
}

fn main() {}
