// https://docs.rs/sled/0.31.0/sled/

use std::str;
use sled;

fn main() -> Result<(), sled::Error> {
    let t = sled::open("email_db").unwrap();

    let target = &t.get(b"steady@learner.com").unwrap().unwrap();

    t.insert(b"steady@learner.com", b"Steadylearner")?; // (Email, Company)
    println!("{:#?}", str::from_utf8(&target).unwrap());

    Ok(())
}
