#[get("/author")]
pub fn webservice() -> Result<String, Box<dyn std::error::Error>> {
    
    let author = "https://www.steadyleaner.com";

    Ok(author.to_string())
}
