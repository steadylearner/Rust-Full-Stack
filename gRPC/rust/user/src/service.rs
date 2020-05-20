use chrono::*;
use uuid::Uuid;

use crate::db_connection::establish_connection;

use tonic::{Request, Response, Status};

use crate::user::{
    user_service_server::UserService, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
};

#[derive(Default)]
pub struct User {}

// If you want optional values or custom error messages, refer to this.
// https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3/42634681
// Should write similar logics similar to delete_user, delete_users functions
#[tonic::async_trait]
impl UserService for User {
    // It works.
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let UserRequest { id } = &request.into_inner();

        let conn = establish_connection();

        let rows = &conn
            .query("SELECT * FROM users WHERE id = $1", &[&id])
            .unwrap();

        let row = rows.get(0);
        // println!("{:#?}", &row);

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
        let conn = establish_connection();

        // https://docs.rs/postgres/0.15.2/postgres/
        // use functional approach? https://docs.rs/postgres/0.15.2/postgres/rows/struct.Rows.html#method.iter
        let mut v: Vec<UserReply> = Vec::new();
        // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
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

        let reply = Users { users: v };

        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // https://crates.io/crates/uuid
        let user_id = Uuid::new_v4().to_hyphenated().to_string();
        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let conn = establish_connection();
        // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.execute
        let number_of_rows_affected = &conn.execute(
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
        println!("Got a request: {:#?}", &request);
        // https://crates.io/crates/uuid
        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();

        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap(); // String to Date

        let conn = establish_connection();

        let number_of_rows_affected = &conn
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
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let conn = establish_connection();

        let number_of_rows_affected = &conn
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .unwrap();

        let reply = if number_of_rows_affected == &(0 as u64) {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
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
        println!("Got a request: {:#?}", &request);
        let conn = establish_connection();

        let rows = &conn.query("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!("Remove {} user data from the database.", rows.len()),
        };

        Ok(Response::new(reply))
    }
}
