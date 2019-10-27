// https://docs.rs/postgres/0.15.2/postgres/
extern crate postgres;
use postgres::{Connection, TlsMode};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

extern crate chrono;
use chrono::*;

use tonic::{transport::Server, Request, Response, Status};

pub mod user {
    tonic::include_proto!("user");
}

use user::{
    server::{Crud, CrudServer},
    UserReply, UserRequest,
    Empty, Users,
};

#[derive(Default)]
pub struct User {}

#[tonic::async_trait]
impl Crud for User {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let user_id = &request.into_inner().id;

        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let conn = Connection::connect(database_url, TlsMode::None).unwrap();

        // unwrap() instead of ?,
        // if you want to handle more
        // you may separate this to function and should define types for them.
        let rows = &conn
            .query("SELECT * FROM users WHERE id = $1", &[&user_id])
            .unwrap();

        // println!("{:#?}", rows);
        // println!("{:#?}", rows.get(0));
        // https://docs.rs/postgres/0.17.0-alpha.1/postgres/row/struct.Row.html

        let row = rows.get(0); // row
        println!("{:#?}", &row);

        // https://github.com/chronotope/chrono
        // https://docs.rs/postgres/0.17.0-alpha.1/postgres/types/trait.FromSql.html#tymethod.from_sql
        // cannot infer type
        // the trait `postgres::types::FromSql` is not implemented for

        let date_of_birth: NaiveDate = row.get(3);

        let reply = UserReply {
            id: row.get(0),
            first_name: row.get(1),
            last_name: row.get(2),
            // https://docs.rs/postgres/0.17.0-alpha.1/postgres/types/trait.FromSql.html?search=to_string
            date_of_birth: date_of_birth.to_string(),
        };

        Ok(Response::new(reply))
    }

    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);

        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = Connection::connect(database_url, TlsMode::None).unwrap();

        // https://docs.rs/postgres/0.15.2/postgres/
        // use functional approach?
        let mut v: Vec<UserReply> = Vec::new();
        for row in &conn.query("SELECT * FROM users", &[]).unwrap() {
            let date_of_birth: NaiveDate = row.get(3);
            let user = UserReply {
                id: row.get(0),
                first_name: row.get(1),
                last_name: row.get(2),
                date_of_birth: date_of_birth.to_string(),
            };
            v.push(user);
        }

        // message Users {
        //     repeated UserReply users = 1;
        // }
        // (repeated, vec), (users, users)

        let reply = Users {
            users: v,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = User::default();

    Server::builder().serve(addr, CrudServer::new(user)).await?;
    Ok(())
}
