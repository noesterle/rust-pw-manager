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

    pub fn open_db(filepath: &String) {
        use std::convert;
        let path = Path::new(filepath);
        let conn: rusqlite::Connection = Connection::open(&path).expect("Could not open a connection to the database.");
        let result = conn.execute("CREATE TABLE IF NOT EXISTS user (usename TEXT, password TEXT);",&[]).expect("Unable to create table.");/*{
            //Ok(tuple) => {for item in tuple.into_iter(){print!("{}",item);}},
            //Ok(tuple) => {tuple.map(|x,y|{ println!("X: {}, Y: {}",x,y})},
            Ok(_) => println!("No error while creatin table."),
            Err(_) => println!("Error while creating table."),
        };*/
        println!("TEST TEST");
        println!("Results: {}",result);
    }
}
