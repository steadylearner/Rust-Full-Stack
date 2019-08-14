use crate::reqwest;

// https://doc.rust-lang.org/beta/std/fs/fn.copy.html
// https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_utf16
// https://doc.rust-lang.org/stable/std/char/fn.from_u32.html

#[test]
pub fn write() -> Result<(), Box<dyn std::error::Error>> {

    let target = "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md";

    let mut res = reqwest::get(target)?;
    let mut body: Vec<u8> = vec![];

    std::io::copy(&mut res, &mut body)?;

    // let characters: Vec<char> = body.into_iter().map(|x| x as char).collect();
    // let result: String = characters.into_iter().collect();

    let result: String = String::from_utf8(body).unwrap();

    println!("{:#?}", &result); // cargo test -- --nocapture

    Ok(())
}
