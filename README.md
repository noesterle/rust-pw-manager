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
While the base functionality is being build, it will ask you to connect to an SQLite3 database. If the database did not exist, it will create the database, ask for a master password, and create the tables. Next, it will ask you to enter information to enter into a password entry. 
This can be stopped by typing the termination string, `!stop`, for any entry. If the termination string is entered, the entry will not be added to the database, and the program will currently exit. 
If all user information was entered successfully, it will be added to the specified database and the program will exit.
