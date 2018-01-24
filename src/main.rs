extern crate xdg;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;

use std::env;

embed_migrations!();

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("til").unwrap();
    let data_file = xdg_dirs.place_data_file("data.sqlite3").unwrap();
    let data_file_str = data_file.to_str().unwrap();
    let connection = SqliteConnection::establish(&data_file_str).unwrap();
    embedded_migrations::run(&connection).unwrap();

    let mut args = env::args();

    if let Some(cmd) = args.nth(1) {
        if cmd == "new" {
            if let Some(contents) = args.nth(0) {
                println!("Going to insert {} til", contents);
            } else {
                println!("Don't forget to provide contents");
            }
        } else {
            println!("Only cmd currently is \"new\"");
        }
    } else {
        println!("Don't forget to provide a command");
    }
}
