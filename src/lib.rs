pub mod crypto {
    pub fn crypto_mod_test(){
        println!("Crypto Mod Test");
    }
}

pub mod sql {
    extern crate rusqlite;
    use std::path::Path;

    use self::rusqlite::Connection;

    pub fn sql_mod_test(){
        println!("SQL Mod Test");
    }

    pub fn open_db(filepath: &String) -> Connection {
        use std::convert;
        let path = Path::new(filepath);
        //TODO see if the db exists, to take user creds if not.
        let mut db_exists = true;
        if !path.exists() { //TODO see if the is_file check should be there.
            db_exists = false;
        }
        let conn: rusqlite::Connection = Connection::open(&path).expect("Could not open a connection to the database.");
        conn.execute("CREATE TABLE IF NOT EXISTS user (password TEXT);",&[]).expect("Unable to create table.");
        if !db_exists {
            use std::io;
            println!("Enter a password for this database.");
            let mut password: String = String::new();
            io::stdin().read_line(&mut password).expect("Not a valid string for a password.");
            password = password.trim().to_string();
            insert_user(&conn, &password);
        }
        return conn;
    }

    fn insert_user(conn: &Connection, pass:&String) {
        conn.execute("INSERT into user(password) VALUES (?)",&[pass]).expect("Could not add password to the user table.");
    }
}
