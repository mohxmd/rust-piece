// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Naruto".to_owned(),
            locker_assignment: Some(30021),
        },
        Student {
            name: "Itachi".to_owned(),
            locker_assignment: None,
        },
        Student {
            name: "Sasuke".to_owned(),
            locker_assignment: Some(10012),
        },
    ];

    for student in students {
        println!("Student");
        println!("Name: {}", student.name);

        match student.locker_assignment {
            Some(locker) => println!("Locker: {}", locker),
            None => println!("Locker: No locker assigned"),
        }

        println!("------------------------");
    }
}
