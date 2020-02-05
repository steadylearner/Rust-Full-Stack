use redis::{
    Client as RedisClient,
    Commands,
    RedisResult,
};

pub fn list_of_users() -> RedisResult<()> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    // throw away the result, just make sure it does not fail
    let _ : () = con.del("users")?;
    Ok(())
}

pub fn user(id: String) -> RedisResult<()> {
    // Extract the Redis target database to variable later.
    let client = RedisClient::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    // Should read the documenation and edit here first.
    let _ : () = con.del(&id)?;
    Ok(())
}

