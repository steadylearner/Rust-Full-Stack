use std::fmt;

use redis::{
    Client as RedisClient,
    Commands,
    FromRedisValue, from_redis_value,
    Value, RedisResult,
};

use serde::{
    de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess},
};

use crate::user::{
    UserReply
};

pub fn list_of_users() -> RedisResult<String> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    let users = con.get("users")?;
    println!("{:#?}", &users);
    Ok(users)
}

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

impl FromRedisValue for UserReply {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let user_reply_from_redis: UserReply = from_redis_value(v)?;
        Ok(user_reply_from_redis)
    }
}

// Is there a better way? Instead of manually implementing it.
// I can't easily find how to use Derive[Serialize Deserialize] for the UserReply type from gRPC.
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

pub fn user(id: String) -> RedisResult<String> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    let user = con.get(&id)?;
    println!("{:#?}", &user);
    Ok(user)
}

