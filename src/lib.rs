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
        let conn: rusqlite::Connection = Connection::open(&path).expect("Could not open a connection to the database.");
        let result = conn.execute("CREATE TABLE IF NOT EXISTS user (usename TEXT, password TEXT);",&[]).expect("Unable to create table.");
        return conn;
    }
}
