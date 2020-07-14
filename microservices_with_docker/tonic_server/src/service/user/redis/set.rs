use redis::{
    Client as RedisClient,
    Commands,
    RedisResult,
    ToRedisArgs, RedisWrite,
};

use crate::{
    user::{
        UserReply
    },
};

use serde_json;

pub fn list_of_users(users: Vec<UserReply>) -> RedisResult<()> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    // The input and output of Redis database should be serialized.
    let serialized = serde_json::to_string(&users).unwrap();
    let _ : () = con.set("users", serialized)?;
    Ok(())
}

impl ToRedisArgs for UserReply {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        let serialized = serde_json::to_string(&*self).unwrap();
        out.write_arg(&serialized.as_bytes());
    }
}

// Use it to create data in Redis.
pub fn user(id: String, user: UserReply) -> RedisResult<()> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    // The input and output of Redis database should be serialized.
    let serialized = serde_json::to_string(&user).unwrap();
    let _ : () = con.set(&id, serialized)?;
    Ok(())
}

