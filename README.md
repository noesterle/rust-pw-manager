# rust-pw-manager
This is a password manager written in Rust.
It currently is designed to be used from the command line.
## Dependencies
This project is designed to use `ring 0.12.1` to secure the information entered into this application. 
It uses `rusqlite 0.12.0` to store the information using SQLite 3. 
It also uses `rpassword 2.0.0` to hide sensitive information entered through the command line.

The correct versions of `ring`, `rusqlite`, and `rpassword` are all automatically downloaded, built, and linked when building or running rust-pw-manager using  `Cargo`. 
This is done by executing `cargo build` or `cargo run` respectively, in or below the `rust-pw-manager/` directory.

## Usage
To start up the application, either build the application using `cargo build` and run the resulting executable or run `cargo run`. 

To begin, this program will ask you to connect to an SQLite3 database. If the database did not exist, it will create the database, ask for a master password, and create the table to store password entries.

Once a database is connected to, the user will be prompted to add a password entry, search through the database, or delete a password entry or quit the program. This will be done by entering the first letter of the first three options ('a', 's', or 'd') or `!quit` to quit the program.
The user should then follow the new prompts to complete the selected action, which includes entering `!stop` to stop the selected action before it is executed. The user will then be prompted to enter another action to perform.
