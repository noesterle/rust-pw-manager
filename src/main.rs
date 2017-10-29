extern crate mylib;

//Custom crate
use mylib::crypto;
use mylib::sql;

//Built in crates
use std::io;

fn main() {
    let mut filepath = String::new();
    println!("Enter a path to the Password Database.\nNote: This path can be relational from the current location, or an absolute filepath.");
    io::stdin().read_line(&mut filepath).expect("Not a string.");
    filepath = filepath.to_string().trim().to_string();
    println!("Filepath: {}", filepath);
    sql::open_db(&filepath);
    println!("user table added to {}",&filepath);
}
