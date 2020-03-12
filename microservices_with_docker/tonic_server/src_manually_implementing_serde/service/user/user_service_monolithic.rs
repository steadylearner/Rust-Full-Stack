use std::fmt;
use chrono::*;
use console::Style;

use uuid::Uuid;
use redis::{
    Client as RedisClient,
    Commands,
    FromRedisValue, from_redis_value,
    Value, RedisResult,
    ToRedisArgs, RedisWrite,
};

// Improve it.(Use something else instead of using 'extern crate serde' directly here)
extern crate serde;
use serde::{
    ser::{Serialize, Serializer, SerializeStruct},
    de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess},
};
use serde_json;

use crate::db_connection::{
    establish_postgres_connection,
};

use tonic::{Request, Response, Status};

use crate::user::{
    user_service_server::UserService, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty, UpdateUserReply,
    UpdateUserRequest, UserReply, UserRequest, Users,
};

// https://serde.rs/remote-derive.html

// Separate it to redis/ later.

// From the author,
// "This trait is well supported throughout the library and
// you can implement it for your own types if you want."

// [Previous thoughts]
// 1. I can implement it for the types I want to use.
//      -> If you can't make it, read the source code of other library authors
//         who uses it or make wrappers for it.
// 2. Or serialize it to save and deserialize before you return it to users.
//      -> It would be the last option when the author says you can solve the problem with the crate.

// -> I need to use both of them to make this compile.

impl UserReply {
    fn new(id: String, first_name: String, last_name: String, date_of_birth: String) -> UserReply {
        UserReply {
            id,
            first_name,
            last_name,
            date_of_birth,
        }
    }
}

// I could make it compile by reading the source code of it.
// If you don't impl it, you will see this.
// `redis::FromRedisValue` is not implemented for `user::UserReply`
impl FromRedisValue for UserReply {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let user_reply_from_redis: UserReply = from_redis_value(v)?;
        Ok(user_reply_from_redis)
    }
}

impl<'de> Deserialize<'de> for UserReply {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Id,
            FirstName,
            LastName,
            DateOfBirth,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`id` or `first_name` or `last_name` or `date_of_birth`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "id" => Ok(Field::Id),
                            "first_name" => Ok(Field::FirstName),
                            "last_name" => Ok(Field::LastName),
                            "date_of_birth" => Ok(Field::DateOfBirth),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct UserReplyVisitor;

        impl<'de> Visitor<'de> for UserReplyVisitor {
            type Value = UserReply;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct UserReply")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<UserReply, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let first_name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let last_name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let date_of_birth = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(UserReply::new(id, first_name, last_name, date_of_birth))
            }

            fn visit_map<V>(self, mut map: V) -> Result<UserReply, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut first_name = None;
                let mut last_name = None;
                let mut date_of_birth = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::FirstName => {
                            if first_name.is_some() {
                                return Err(de::Error::duplicate_field("first_name"));
                            }
                           first_name = Some(map.next_value()?);
                        }
                        Field::LastName => {
                            if last_name.is_some() {
                                return Err(de::Error::duplicate_field("last_name"));
                            }
                            last_name = Some(map.next_value()?);
                        }
                        Field::DateOfBirth => {
                            if date_of_birth.is_some() {
                                return Err(de::Error::duplicate_field("date_of_birth"));
                            }
                            date_of_birth = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let first_name = first_name.ok_or_else(|| de::Error::missing_field("first_name"))?;
                let last_name = last_name.ok_or_else(|| de::Error::missing_field("last_name"))?;
                let date_of_birth = date_of_birth.ok_or_else(|| de::Error::missing_field("date_of_birth"))?;
                Ok(UserReply::new(id, first_name, last_name, date_of_birth,))
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "first_name", "last_name", "date_of_birth"];
        deserializer.deserialize_struct("UserReply", FIELDS, UserReplyVisitor)
    }
}

fn get_list_of_users() -> redis::RedisResult<String> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;
    let steady = con.get("steady")?;
    println!("{:#?}", &steady);

    let users = con.get("users")?;
    println!("{:#?}", &users);
    Ok(users)
}

impl ToRedisArgs for UserReply {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        // Serialize it with serde?
        // Then, turn it to byte?
        let serialized = serde_json::to_string(&*self).unwrap();
        out.write_arg(&serialized.as_bytes()); // expected `&[u8]`, found struct `user::Users`
    }
}

impl Serialize for UserReply {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserReply", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("date_of_birth", &self.date_of_birth)?;
        state.end()
    }
}

fn set_list_of_users(users: Vec<UserReply>) -> redis::RedisResult<()> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    // The input and output of Redis database should be serialized.
    let serialized = serde_json::to_string(&users).unwrap();
    let _ : () = con.set("users", serialized)?;
    Ok(())
}

#[derive(Default)]
pub struct User {}

// If you want optional values or custom error messages, refer to this.
// https://stackoverflow.com/questions/42622015/how-to-define-an-optional-field-in-protobuf-3/42634681
// Should write similar logics similar to delete_user, delete_users functions
#[tonic::async_trait]
impl UserService for User {
    // 1. Make this pass first. Then, others will be easy.
    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);
        let postgres_conn = establish_postgres_connection();

        let blue = Style::new()
            .blue();
        let red = Style::new()
            .red();
        let redis = "[Redis]";
        let postgresql = "[Postgresql]";

        //  list_users().is_ok() -> 1. There was a cache data in Redis. Then, return it.
        //  else -> 2. There was no data in Redis or problem with it. Return the postgresl data to user. Then, save it to Redis also.
        let reply = match get_list_of_users() {
            Ok(list_of_users_from_redis) => { // It returns || when there is no key set yet. Therefore,  use this logic.
                if !list_of_users_from_redis.is_empty() { // users.len == 0
                    // 2.
                    println!("\n{:#?}", red.apply_to(&redis));
                    println!("{:#?}", &list_of_users_from_redis);
                    // The input and output of Redis database should be serialized.
                    // When, you need the data from it, deserialize it.
                    let users: Vec<UserReply> = serde_json::from_str(&list_of_users_from_redis).unwrap();
                    let data_from_redis = Users { users };
                    data_from_redis
                } else {
                    println!("\n{:#?}", blue.apply_to(&postgresql));

                    // https://docs.rs/postgres/0.15.2/postgres/
                    // use functional approach? https://docs.rs/postgres/0.15.2/postgres/rows/struct.Rows.html#method.iter
                    let mut v: Vec<UserReply> = Vec::new();
                    // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
                    for row in &postgres_conn.query("SELECT * FROM users", &[]).unwrap() {
                        let date_of_birth: NaiveDate = row.get(3);
                        let user = UserReply {
                            id: row.get(0),
                            first_name: row.get(1),
                            last_name: row.get(2),
                            date_of_birth: date_of_birth.to_string(),
                        };
                        v.push(user);
                    }

                    // 1. Make set work first and test get part.
                    let result = set_list_of_users(v.clone());
                    println!("The result of set_list_of_users is {:#?}", result);

                    let data_from_postgres = Users { users: v };
                    data_from_postgres
                }

            },
            Err(e) => {
                println!("{:#?}", e);
                println!("\n{:#?}", blue.apply_to(&postgresql));

                // https://docs.rs/postgres/0.15.2/postgres/
                // use functional approach? https://docs.rs/postgres/0.15.2/postgres/rows/struct.Rows.html#method.iter
                let mut v: Vec<UserReply> = Vec::new();
                // https://docs.rs/postgres/0.15.2/postgres/struct.Connection.html#method.query
                for row in &postgres_conn.query("SELECT * FROM users", &[]).unwrap() {
                    let date_of_birth: NaiveDate = row.get(3);
                    let user = UserReply {
                        id: row.get(0),
                        first_name: row.get(1),
                        last_name: row.get(2),
                        date_of_birth: date_of_birth.to_string(),
                    };
                    v.push(user);
                }

                let result = set_list_of_users(v.clone());
                println!("{:#?}", result);

                let data_from_postgres = Users { users: v };
                data_from_postgres
            }
        };

        Ok(Response::new(reply))
    }

    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let UserRequest { id } = &request.into_inner();

        let postgres_conn = establish_postgres_connection();

        let rows = &postgres_conn
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
        let postgres_conn = establish_postgres_connection();

        let number_of_rows_affected = &postgres_conn
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
        let postgres_conn = establish_postgres_connection();

        let rows = &postgres_conn.query("DELETE FROM users", &[]).unwrap();

        let reply = DeleteUserReply {
            message: format!("Remove {} user data from the database.", rows.len()),
        };

        Ok(Response::new(reply))
    }
}
