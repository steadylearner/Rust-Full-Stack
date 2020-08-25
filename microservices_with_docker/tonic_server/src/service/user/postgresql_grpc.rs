use chrono::NaiveDate;
use console::Style;

use uuid::Uuid;

use serde_json;

use crate::db_connection::{
    establish_postgres_connection,
    // Use Model or separate it from db_connection.rs
    query_list_of_users,
};

use tonic::{Request, Response, Status};

use crate::user::{
    user_service_server::UserService, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
};

use super::redis::{
    get,
    set,
    delete,
};

#[derive(Default)]
pub struct User {}

// If you want optional values or custom error messages, refer to this.
// https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3/42634681
// Should write similar logics similar to delete_user, delete_users functions
#[tonic::async_trait]
impl UserService for User {
    // 1. Make this pass first. Then, others will be easy.
    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        // Extract it to function with println!
        let blue = Style::new()
            .blue();
        let postgresql = "[Postgresql]";

        // Extract it to function with println!
        let red = Style::new()
            .red();
        let redis = "[Redis]";

        println!("Receive a request: {:#?}", &request);
        let postgres_conn = establish_postgres_connection();

        //  list_users().is_ok() -> 1. There was a cache data in Redis. Then, return it.
        //  else -> 2. There was no data in Redis or problem with it. Return the postgresl data to user. Then, save it to Redis also.
        let reply = match get::list_of_users() {
            Ok(list_of_users_from_redis) => {
                if !list_of_users_from_redis.is_empty() { // If there is no data in redis or error, use postgresql.
                    println!("\n{:#?}", red.apply_to(&redis));
                    println!("{:#?}\n", &list_of_users_from_redis);
                    // The input and output of Redis database should be serialized.
                    // When, you need the data from it, deserialize it.
                    let users: Vec<UserReply> = serde_json::from_str(&list_of_users_from_redis).unwrap();
                    let data_from_redis = Users { users };
                    data_from_redis
                } else {
                    println!("\n{:#?}", blue.apply_to(&postgresql));

                    let v: Vec<UserReply> = query_list_of_users(postgres_conn);
                    let data_for_redis = v.clone();
                    let result = set::list_of_users(data_for_redis);
                    println!("The result of set::list_of_users is {:#?}", result);

                    let data_from_postgres = Users { users: v };
                    data_from_postgres
                }

            },
            Err(e) => {
                println!("{:#?}", e);
                println!("\n{:#?}", blue.apply_to(&postgresql));

                let v: Vec<UserReply> = query_list_of_users(postgres_conn);
                let data_for_redis = v.clone();
                let result = set::list_of_users(data_for_redis);
                println!("The result of set::list_of_users is {:#?}", result);

                let data_from_postgres = Users { users: v };
                data_from_postgres
            }
        };

        Ok(Response::new(reply))
    }

    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        let blue = Style::new()
            .blue();
        let red = Style::new()
            .red();
        let redis = "[Redis]";
        let postgresql = "[Postgresql]";

        println!("Receive a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let UserRequest { id } = &request.into_inner();
        let postgres_conn = establish_postgres_connection();

        let reply = match get::user(id.to_string()) {
            Ok(user_from_redis) => {
                if user_from_redis != "()" { // If there is no data in redis or error, use postgresql.
                    println!("\n{:#?}", red.apply_to(&redis));
                    println!("{:#?}\n", &user_from_redis);
                    let data_from_redis: UserReply = serde_json::from_str(&user_from_redis).unwrap();
                    data_from_redis
                } else {
                    println!("\n{:#?}", blue.apply_to(&postgresql));

                    let rows = &postgres_conn
                        .query("SELECT * FROM users WHERE id = $1", &[&id])
                        .unwrap();

                    let row = rows.get(0);
                    // println!("{:#?}", &row);

                    // Extract it to function.
                    let date_of_birth: NaiveDate = row.get(3);

                    let data_from_postgres = UserReply {
                        id: row.get(0),
                        first_name: row.get(1),
                        last_name: row.get(2),
                        // https://docs.rs/postgres/0.17.0-alpha.1/postgres/types/trait.FromSql.html?search=to_string
                        date_of_birth: date_of_birth.to_string(),
                    };

                    let data_for_redis = data_from_postgres.clone();
                    let result = set::user(data_for_redis.id.clone(), data_for_redis);
                    println!("The result of set::user({}) is {:#?}", id, result);

                    data_from_postgres
                }

            },
            Err(e) => {
                println!("{:#?}", e);
                println!("\n{:#?}", blue.apply_to(&postgresql));

                let rows = &postgres_conn
                    .query("SELECT * FROM users WHERE id = $1", &[&id])
                    .unwrap();

                let row = rows.get(0);
                // println!("{:#?}", &row);

                // Extract it to function.
                let date_of_birth: NaiveDate = row.get(3);

                let data_from_postgres = UserReply {
                    id: row.get(0),
                    first_name: row.get(1),
                    last_name: row.get(2),
                    // https://docs.rs/postgres/0.17.0-alpha.1/postgres/types/trait.FromSql.html?search=to_string
                    date_of_birth: date_of_birth.to_string(),
                };

                let data_for_redis = data_from_postgres.clone();
                let result = set::user(data_for_redis.id.clone(), data_for_redis);
                println!("The result of set::user({}) is {:#?}", id, result);

                data_from_postgres
            }
        };

        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Receive a request: {:#?}", &request);
        // https://crates.io/crates/uuid
        let user_id = Uuid::new_v4().to_hyphenated().to_string();
        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let postgres_conn = establish_postgres_connection();
        // https://docs.rs/postgres/0.15.2/postgres/struct.postgres_connection.html#method.execute
        let number_of_rows_affected = &postgres_conn.execute(
                "INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ($1, $2, $3, $4)",
                &[
                    &user_id,
                    &first_name,
                    &last_name,
                    &serialize_date_of_birth,
                ]
            )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            CreateUserReply {
                message: format!(
                    "Fail to create user with id {}.",
                    &user_id
                ),
            }
        } else {
            let red = Style::new()
                .red();
            let redis = "[Redis]";
            println!("\n{:#?}", red.apply_to(&redis));

            let data_for_redis = UserReply {
                id: user_id.to_string(),
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                date_of_birth: date_of_birth.to_string(),
            };

            println!("Redis would save the user {} with the data {:#?}", &user_id, &data_for_redis);
            let result = set::user(data_for_redis.id.clone(), data_for_redis);
            println!("The result of set::user({}) is {:#?}", &user_id, &result);

            CreateUserReply {
                message: format!(
                    "Create {} user with id {}.",
                    &number_of_rows_affected, &user_id
                ),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Receive a request: {:#?}", &request);
        // https://crates.io/crates/uuid
        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();

        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let postgres_conn = establish_postgres_connection();

        let number_of_rows_affected = &postgres_conn
            .execute(
                "UPDATE users SET first_name = $2, last_name = $3, date_of_birth = $4 WHERE id = $1",
                &[
                    &id,
                    &first_name,
                    &last_name,
                    &serialize_date_of_birth,
                ]
            )
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", id),
            }
        } else {
            let red = Style::new()
                .red();
            let redis = "[Redis]";
            println!("\n{:#?}", red.apply_to(&redis));

            let data_for_redis = UserReply {
                id: id.to_string(),
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                date_of_birth: date_of_birth.to_string(),
            };

            println!("Redis would update the user {} with the data {:#?}", &id, &data_for_redis);
            let result = set::user(data_for_redis.id.clone(), data_for_redis);
            println!("The result of set::user({}) is {:#?}", &id, &result);

            UpdateUserReply {
                message: format!("Update {} user with id {}.", &number_of_rows_affected, &id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Receive a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let postgres_conn = establish_postgres_connection();

        let number_of_rows_affected = &postgres_conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
            let red = Style::new()
                .red();
            let redis = "[Redis]";
            println!("\n{:#?}", red.apply_to(&redis));

            println!("Redis should remove the user with id {}", &id);
            let result = delete::user(id.to_string());
            // There should be a better way to handle the result.
            println!("The result of delete::user({}) is {:#?}", &id, &result);

            DeleteUserReply {
                message: format!("Remove the user with id {}.", id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Receive a request: {:#?}", &request);
        let postgres_conn = establish_postgres_connection();

        let rows = &postgres_conn.query("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!("Remove {} user data from the database.", rows.len()),
        };

        let red = Style::new()
            .red();
        let redis = "[Redis]";
        println!("\n{:#?}", red.apply_to(&redis));

        println!("Redis should remove all the users");
        let result = delete::list_of_users();
        // There should be a better way to handle the result.
        println!("The result of delete::list_of_users() is {:#?}", &result);

        Ok(Response::new(reply))
    }
}
