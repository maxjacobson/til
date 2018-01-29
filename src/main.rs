extern crate xdg;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::prelude::*;

use std::env;

pub mod schema;
pub mod models;

use schema::tils;
use models::{NewTIL, TIL};

embed_migrations!();

fn main() {
    let data_file_path_buf = match env::var("TIL_FILE") {
        Ok(non_default_path) => std::path::PathBuf::from(non_default_path),
        Err(env::VarError::NotPresent) => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix("til").unwrap();
            xdg_dirs.place_data_file("data.sqlite3").unwrap()
        }
        Err(env::VarError::NotUnicode(_)) => panic!("TIL_FILE is not valid unicode"),
    };

    let data_file_str = data_file_path_buf.to_str().unwrap();
    let connection = SqliteConnection::establish(&data_file_str).unwrap();

    // TODO: log this to a file instead of stdout
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let mut args = env::args();

    if let Some(cmd) = args.nth(1) {
        if cmd == "new" {
            if let Some(contents) = args.nth(0) {

                let new_til = NewTIL { contents: &contents };

                diesel::insert_into(tils::table)
                    .values(&new_til)
                    .execute(&connection)
                    .unwrap();

                println!("Inserted new til: {:?}", contents);
            } else {
                println!("Don't forget to provide contents for your new til");
            }
        } else if cmd == "list" {
            {
                use schema::tils::dsl::*;

                let results = tils.order(created_at.desc())
                    .load::<TIL>(&connection)
                    .unwrap();

                let len = results.len();

                for til in results {
                    println!("{} - {}", til.created_at, til.contents);
                }

                if len == 0 {
                    println!("None yet. Why not create one?");
                }
            }
        } else {
            println!("Only subcommands currently are \"new\" and \"list\"");
        }
    } else {
        println!("Don't forget to provide a command. Usage: `til new 'Rust is cool'`");
    }
}
