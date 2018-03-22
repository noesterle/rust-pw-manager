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
    let conn = sql::open_db(&filepath);

    let mut action = String::new();
    let mut run = true;

    while run {
        println!("Enter an action to take: 'a' to add an Entry, 's' to search the database, 
                 'd' to delete an entry, or '!quit' to quit the program.");
        io::stdin().read_line(&mut action).expect("Unable to read action.");
        action = action.trim().to_string();

        if action.to_uppercase() == "A" {
            sql::insert_entry(&conn);
        }
        else if action.to_uppercase() == "S" {
            sql::search_entry(&conn);
        }
        else if action.to_uppercase() == "D" {
            sql::delete_entry(&conn);
        }
        else if action == "!quit" {
            run = false;
        }
        action.clear();
    }
}
