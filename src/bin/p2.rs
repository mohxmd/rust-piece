// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records {
    inner: HashMap<i64, Record>,
}

impl Records {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.inner.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email,
            },
        );
    }

    fn next_id(&self) -> i64 {
        let mut ids: Vec<_> = self.inner.keys().copied().collect();
        ids.sort();
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }

    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }

    fn into_vec(mut self) -> Vec<Record> {
        let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
        records.sort_by_key(|rec| rec.id);
        records
    }

    fn search(&self, name: &str) -> Vec<&Record> {
        let query = name.to_lowercase();
        self.inner
            .values()
            .filter(|rec| rec.name.to_lowercase().contains(&query))
            .collect()
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("id must be a number: {0}")]
    InvalidId(#[from] std::num::ParseIntError),
    #[error("empty record")]
    EmptyRecord,
    #[error("missing field: {0}")]
    MissingField(String),
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = record.split(',').collect();

    let id = match fields.first() {
        Some(id) if !id.trim().is_empty() => i64::from_str_radix(id.trim(), 10)?,
        Some(_) => return Err(ParseError::MissingField("id".to_string())),
        None => return Err(ParseError::EmptyRecord),
    };

    let name = match fields.get(1).map(|name| name.trim()).filter(|name| !name.is_empty()) {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

    let email = fields
        .get(2)
        .map(|email| email.trim().to_string())
        .filter(|email| !email.is_empty());

    Ok(Record { id, name, email })
}

fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    for (num, record) in records.lines().enumerate() {
        let trimmed = record.trim();
        if trimmed.is_empty() || trimmed == "id,name,email" {
            continue;
        }

        match parse_record(trimmed) {
            Ok(rec) => recs.add(rec),
            Err(e) => {
                if verbose {
                    println!(
                        "error on line number {}: {}\n  > \"{}\"\n",
                        num + 1,
                        e,
                        record
                    );
                }
            }
        }
    }
    recs
}

fn load_records(file_name: PathBuf, verbose: bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(parse_records(buffer, verbose))
}

fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write_all(b"id,name,email\n")?;

    for record in records.into_vec() {
        let email = record.email.unwrap_or_default();
        let line = format!("{},{},{}\n", record.id, record.name, email);
        file.write_all(line.as_bytes())?;
    }

    file.flush()?;
    Ok(())
}

enum Command {
    Add { name: String, email: Option<String> },
    Edit { id: i64, name: String, email: Option<String> },
    List,
    Remove { id: i64 },
    Search { query: String },
}

struct Opt {
    data_file: PathBuf,
    cmd: Command,
    verbose: bool,
}

fn parse_args() -> Result<Opt, String> {
    let mut args = env::args().skip(1).peekable();
    let mut data_file = PathBuf::from("src/bin/p2_data.csv");
    let mut verbose = false;

    while let Some(arg) = args.peek().cloned() {
        match arg.as_str() {
            "-v" | "--verbose" => {
                verbose = true;
                args.next();
            }
            "-f" | "--file" => {
                args.next();
                let value = args
                    .next()
                    .ok_or_else(|| "missing file path after --file".to_string())?;
                data_file = PathBuf::from(value);
            }
            _ => break,
        }
    }

    let command = args
        .next()
        .ok_or_else(|| usage("missing command"))?;

    let cmd = match command.as_str() {
        "list" => Command::List,
        "search" => {
            let query = args
                .next()
                .ok_or_else(|| usage("missing query for search"))?;
            Command::Search { query }
        }
        "remove" => {
            let id = parse_id(args.next(), "missing id for remove")?;
            Command::Remove { id }
        }
        "add" => {
            let name = args.next().ok_or_else(|| usage("missing name for add"))?;
            let email = args.next();
            Command::Add { name, email }
        }
        "edit" => {
            let id = parse_id(args.next(), "missing id for edit")?;
            let name = args.next().ok_or_else(|| usage("missing name for edit"))?;
            let email = args.next();
            Command::Edit { id, name, email }
        }
        _ => return Err(usage("unknown command")),
    };

    Ok(Opt {
        data_file,
        cmd,
        verbose,
    })
}

fn parse_id(value: Option<String>, missing_msg: &str) -> Result<i64, String> {
    let raw = value.ok_or_else(|| usage(missing_msg))?;
    raw.parse::<i64>()
        .map_err(|_| format!("invalid id: {raw}"))
}

fn usage(error: &str) -> String {
    format!(
        "{error}\nusage:\n  p2 [--verbose] [--file path] list\n  p2 [--verbose] [--file path] search <query>\n  p2 [--verbose] [--file path] add <name> [email]\n  p2 [--verbose] [--file path] edit <id> <name> [email]\n  p2 [--verbose] [--file path] remove <id>"
    )
}

fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
            println!("record added");
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
            println!("record saved");
        }
        Command::List => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{record:?}");
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("record deleted");
            } else {
                println!("record not found");
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            let results = recs.search(&query);
            if results.is_empty() {
                println!("no records found");
            } else {
                for rec in results {
                    println!("{rec:?}");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    match parse_args() {
        Ok(opt) => {
            if let Err(e) = run(opt) {
                println!("an error occurred: {}", e);
            }
        }
        Err(message) => println!("{message}"),
    }
}
