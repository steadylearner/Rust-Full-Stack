use std::io::stdin;

pub fn str_from_stdin() -> String {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    // Remove \n
    let user_input = user_input[..(user_input.len() - 1)].to_string();

    user_input
}

pub fn default_subject_if_empty(input: String) -> String {
    if input.is_empty() {
        let default_subject = "Rust Developer";
        // println!("{}", &default_subject);
        default_subject.to_string()
    } else {
        input
    }
}

pub fn default_template_if_empty(input: String) -> String {
    if input.is_empty() {
        let default_template = "resume.html";
        // println!("{}", &default_template);
        default_template.to_string()
    } else {
        input
    }
}

// for update from form? &str for remote data such as form, which can't take ownership
pub fn none_if_empty_or_some_str<'a>(input: &'a String) -> Option<&'a str> {
    if input.is_empty() {
        None
    } else {
        Some(&input)
    }
}


