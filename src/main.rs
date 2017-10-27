//extern crate crypto;
//extern crate sql_lib;
extern crate mylib;

//use sql_lib::test;
//use crypto::*;
use mylib::crypto;
use mylib::sql;

fn main() {
    //test();
    //crypto_test();
    crypto::crypto_mod_test();
    sql::sql_mod_test();
    println!("Hello, world!");
}
