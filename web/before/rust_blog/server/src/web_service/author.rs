use std::fs::File;
use std::io::prelude::*;

use std::io::BufReader;
use std::io::Result;

// Read file in server and send the content to user
#[get("/author")]
pub fn webservice() -> Result<String> {
 
    let file_path = format!("posts/author.md");

    let file = File::open(file_path)?;    
    let mut buf_reader = BufReader::new(file);

    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    Ok(content)
}

