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

    use std::io;

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
        //insert_entry(&conn);
        search_entry(&conn);
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
        //Stop if not all the information is not there, meaning the user used the termination
        //string.
        if user_input.len() == columns().len() {
            conn.execute(&format!("INSERT INTO password_entry ({0},{1},{2},{3},{4}) VALUES (?1,?2,?3,?4,?5)",
                entry_columns[0],entry_columns[1],entry_columns[2],entry_columns[3],entry_columns[4]),
                &[&user_input[0],&user_input[1],&user_input[2],&user_input[3],&user_input[4]]);
        }
        else {
            println!("Not all properties were added, so the the password entry was not added.");
        }
    }

    fn user_input() -> Vec<String> {
        use std::io;
        let mut info: Vec<String> = Vec::new(); //TODO make this an array?
        let columns = columns();
        let mut entry = String::new();
        let mut broken = false;
        let stop_keyword = "!stop".to_string();

        //Gather user input for each DB column.
        for item in columns.iter() { //TODO make this a counting for-loop
            println!("Enter the {} for this entry:",item);
            
            //Hide user entry if password is being entered.
            if item == "password" { //TODO is there a way to generalize this?
                let mut different = true;
                let mut confirm = String::new();
                
                //Has user confirm password to cutdown on potential spelling errors.
                while different {
                    entry = rpassword::prompt_password_stdout("Note: The password will be hidden.\n").unwrap();
                    //Stop if user entry enters the termination string.
                    if (entry.trim().to_string() == stop_keyword) {
                        broken = true;
                        break;
                    }
                    confirm = rpassword::prompt_password_stdout("Please confirm your password.\n").unwrap();
                    //Stop if user entry enters the termination string.
                    if (confirm.trim().to_string() == stop_keyword) {
                        broken = true;
                        break;
                    }
                    if (entry == confirm) {
                        different = false;
                    }
                    else {
                        println!("The passwords did not match. Please re-enter your password.");
                    }
                }
            }
            else {
                io::stdin().read_line(&mut entry).expect("Unable to read property.");
            }
            //Remove newlines and store for entry.
            entry = entry.trim().to_string();
            //Stop if user entry enters or has entered the termination string.
            if (entry != stop_keyword && !broken) {
                info.push(entry.clone()); //TODO If this is turned into an array, this will need to be added at an instead of pushed.
            }
            else {
                break;
            }
            entry.clear(); //read_line just appends input, this makes it act like it's overwriting the input.
        }
        return info;
    }

    fn search_entry(conn: &Connection) {
        let cols = columns();
        let mut stmt = conn.prepare(&format!("select * from password_entry where {0} LIKE ? OR {1} LIKE ? OR {2} LIKE ? OR {3} LIKE ?", 
                                             cols[0], cols[1], cols[3], cols[4])).expect("Unable to get password entry.");
        println!("Enter text to search against name's of entries:");
        let mut search_term = String::new();
        io::stdin().read_line(&mut search_term).expect("Not a string.");
        search_term = search_term.to_string().trim().to_string();

        //Create SQL Pattern to search against
        let search_pattern = format!("%{}%",search_term);

        let mut stmt_iter = stmt.query_map(&[&search_pattern,&search_pattern,&search_pattern,&search_pattern],|row|{
            for num in 0..columns().len() as i32 {
                //Need to specify the type used to find the right column in the row and the output type.
                print!("{}  |  ", row.get::<i32,String>(num)); 
            }
            println!("");
        }).unwrap();
        
        //Appears as if the resulting MappedRows need to be used before they can be printed to console. Not really sure why.
        let count = stmt_iter.count(); 
    }

    //fn delete(&conn) {
    //
    //}
}
