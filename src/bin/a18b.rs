// Topic: Result & the question mark operator
//
// Summary:
//   This small program simulates Straw Hat crew members trying to unlock
//   protected ship locations using keycards backed by a database. Many errors
//   can occur when working with a database, making the question mark operator
//   the perfect thing to use to keep the code manageable.
//
// Requirements:
// * Write the body of the `authorize` function. The steps to authorize a pirate
//   are:
//     1. Connect to the database
//     2. Find the pirate with the `find_pirate` database function
//     3. Get a keycard with the `get_keycard` database function
//     4. Determine if the keycard's `access_level` is sufficient, using the
//        `required_access_level` function implemented on `ProtectedLocation`.
//        * Higher `access_level` values grant more access to `ProtectedLocations`.
//          1000 can access 1000 and lower. 800 can access 500 but not 1000, ...
// * Run the program after writing your `authorize` function. Expected output:
//     Ok(Allow)
//     Ok(Deny)
//     Err("Nami doesn't have a keycard")
// * Use the question mark operator within the `authorize` function.

#[derive(Debug)]
enum Authorization {
    Allow,
    Deny,
}
#[derive(Debug, Clone)]
struct Pirate {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_pirate(&self, name: &str) -> Result<Pirate, String> {
        match name {
            "Luffy" => Ok(Pirate {
                name: "Luffy".to_string(),
            }),
            "Usopp" => Ok(Pirate {
                name: "Usopp".to_string(),
            }),
            "Nami" => Ok(Pirate {
                name: "Nami".to_string(),
            }),
            _ => Err(String::from("Pirate not found")),
        }
    }

    fn get_keycard(&self, pirate: &Pirate) -> Result<KeyCard, String> {
        match pirate.name.as_str() {
            "Luffy" => Ok(KeyCard { access_level: 1000 }),
            "Usopp" => Ok(KeyCard { access_level: 500 }),
            "Nami" => Err(format!("{} doesn't have a keycard", pirate.name)),
            _ => Err(String::from("Keycard not found")),
        }
    }
}

enum ProtectedLocation {
    TreasureRoom,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            ProtectedLocation::TreasureRoom => 1000,
        }
    }
}

fn authorize(name: &str, location: ProtectedLocation) -> Result<Authorization, String> {
    let database = Database::connect()?;
    let pirate = database.find_pirate(name)?;
    let keycard = database.get_keycard(&pirate)?;

    if keycard.access_level >= location.required_access_level() {
        Ok(Authorization::Allow)
    } else {
        Ok(Authorization::Deny)
    }
}

fn main() {
    println!("{:?}", authorize("Luffy", ProtectedLocation::TreasureRoom));
    println!("{:?}", authorize("Usopp", ProtectedLocation::TreasureRoom));
    println!("{:?}", authorize("Nami", ProtectedLocation::TreasureRoom));
}
