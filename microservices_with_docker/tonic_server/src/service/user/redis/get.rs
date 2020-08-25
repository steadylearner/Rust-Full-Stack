use redis::{
    Client as RedisClient,
    Commands,
    FromRedisValue, from_redis_value,
    Value, RedisResult,
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

impl FromRedisValue for UserReply {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let user_reply_from_redis: UserReply = from_redis_value(v)?;
        Ok(user_reply_from_redis)
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

