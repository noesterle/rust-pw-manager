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

    fn columns() -> Vec<String> {
        return vec!["name".to_string(),"username".to_string(),"password".to_string(),"url".to_string(),"notes".to_string()];
    }

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
        create_entry_table(&conn);
        insert_entry(&conn);
        return conn;
    }

    fn insert_user(conn: &Connection, pass:&String) {
        conn.execute("INSERT into user(password) VALUES (?)",&[pass]).expect("Could not add password to the user table.");
    }

    fn create_entry_table(conn: &Connection) {
        let entry_columns = columns();
        let result = conn.execute(&format!("CREATE TABLE IF NOT EXISTS password_entry ({0} TEXT, {1} TEXT, {2} TEXT, {3} TEXT, {4} TEXT)",
            entry_columns[0],entry_columns[1],entry_columns[2],entry_columns[3],entry_columns[4]),
            &[]).expect("Unable to create password entry table.");
    }

    fn insert_entry(conn: &Connection) {
        let entry_columns = columns();
        let user_input = user_input();
        conn.execute(&format!("INSERT INTO password_entry ({0},{1},{2},{3},{4}) VALUES (?1,?2,?3,?4,?5)",
            entry_columns[0],entry_columns[1],entry_columns[2],entry_columns[3],entry_columns[4]),
            //&[&"test_name".to_string(),&"test_user".to_string(),&"test_pass".to_string(),&"test_url".to_string(),&"test_notes".to_string()]);
            &[&user_input[0],&user_input[1],&user_input[2],&user_input[3],&user_input[4]]);
    }

    fn user_input() -> Vec<String> {
        use std::io;
        let mut info: Vec<String> = Vec::new(); //TODO make this an array?
        let columns = columns();
        let mut entry = String::new();
        for item in columns.iter() { //TODO make this a counting for-loop
            println!("Enter the {} for this entry:",item);
            io::stdin().read_line(&mut entry).expect("Unable to read property.");
            entry = entry.trim().to_string();
            info.push(entry.clone()); //TODO If this is turned into an array, this will need to be added at an instead of pushed.
            entry.clear();
        }
        return info;
    }
}
