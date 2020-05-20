#[get("/post")]
pub fn webservice() -> Result<String, Box<dyn std::error::Error>> {
    // https://raw.githubusercontent.com/yewstack/yew/master/README.md
    // https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md

    let target = "https://raw.githubusercontent.com/steadylearner/react-easy-md/master/README.md";

    let mut res = reqwest::get(target)?;
    let mut body: Vec<u8> = vec![];

    std::io::copy(&mut res, &mut body)?;

    let result: String = String::from_utf8(body).unwrap();

    println!("{:#?}", &result); 

    Ok(result)
}
