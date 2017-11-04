# rust-pw-manager
This is a password manager written in Rust.
It currently is designed to be used from the command line.
## Dependencies
This project is designed to use `ring 0.12.1` to secure the information entered into this application. 
It uses `rusqlite 0.12.0` to store the information using SQLite 3. 
It also uses `rpassword 2.0.0` to hide sensitive information entered through the command line.

The correct versions of `ring`, `rusqlite`, and `rpassword` are all automatically downloaded, built, and linked when building or running rust-pw-manager using  `Cargo`. 
This is done by executing `cargo build` or `cargo run` respectively, in or below the `rust-pw-manager/` directory.
