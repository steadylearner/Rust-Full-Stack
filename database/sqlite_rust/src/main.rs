use std::io::stdin;

use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

use chrono::naive::NaiveDateTime;

#[derive(Debug)]
struct Message {
    query: String, // Query is unique so use it as id.
    created_at: NaiveDateTime,
}

pub fn from_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input[..(input.len() - 1)].to_string();

    input
}

// Refer to main_crud_prototype.rs
// Make a chat app, web app, bots etc 
fn main() -> Result<()> {
    let conn = Connection::open("messages.db")?;

    conn.execute(
        "create table if not exists messages (
            query text not null unique,
            created_at DATE DEFAULT (datetime('now','localtime'))
        )",
        NO_PARAMS,
    )?;

    println!("Do you want to create or read a message[c, r]");
    let action = from_stdin();

    match action.as_ref() {
        "c" => {
            println!("What do you want to save in messages?");
            let query = from_stdin();
            conn.execute("INSERT INTO messages (query) values (?1)", &[&query])?;
            println!("{:#?} is included in messages.", query)
        }
        "r" => {
            println!("What do you want to read from messages?");
            let query = from_stdin();

            let mut stmt = conn.prepare("SELECT * FROM messages WHERE query = (?1);")?;

            // You can use others instead of query_map.
            // https://docs.rs/rusqlite/0.21.0/rusqlite/struct.Statement.html#method.query
            let message = stmt.query_map(params![&query], |row| {
                Ok(Message {
                    query: row.get(0)?,
                    created_at: row.get(1)?,
                })
            })?;

            for row in message {
                println!("{:#?}", row?);
            }
        }
        _ => {
            println!("You should use [c, r] to create or read messages.")
        }
    };

    Ok(())
}
