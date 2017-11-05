pub mod crypto {
    pub fn crypto_mod_test(){
        println!("Crypto Mod Test");
    }
}

pub mod sql {
    extern crate rpassword;
    extern crate rusqlite;
    use std::path::Path;

    use self::rusqlite::Connection;

    pub fn sql_mod_test(){
        println!("SQL Mod Test");
    }

    pub fn open_db(filepath: &String) -> Connection {
        use std::convert;
        let path = Path::new(filepath);
        
        //see if the db exists, to take user creds if not.
        let mut db_exists = true;
        if !path.exists() { 
            db_exists = false;
        }
        
        //Opening the connection will create the file if it does not exist, or connect to the file
        //if it does.
        let conn: rusqlite::Connection = Connection::open(&path).expect("Could not open a connection to the database.");
        conn.execute("CREATE TABLE IF NOT EXISTS user (password TEXT);",&[]).expect("Unable to create table.");
        
        //If the database did not exist, set the master password for it.
        if !db_exists {
            use std::io;
            println!("Enter a password for this database.\nNote: You will not be able to see the password as you are entering it.");
            let mut password = rpassword::prompt_password_stdout("Password: ").unwrap();
            password = password.trim().to_string();
            insert_user(&conn, &password);
        }
        return conn;
    }

    fn insert_user(conn: &Connection, pass:&String) {
        conn.execute("INSERT into user(password) VALUES (?)",&[pass]).expect("Could not add password to the user table.");
    }
}
