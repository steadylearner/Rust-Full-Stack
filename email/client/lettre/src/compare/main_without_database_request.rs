extern crate console;
extern crate dotenv;
// extern crate serde;
// extern crate serde_json;
extern crate lettre;
// https://docs.rs/mime/0.3.13/mime/index.html
extern crate mime;
extern crate uuid;
use dotenv::dotenv;
use std::env;

use console::Style;
use std::fs;

use lettre::builder::Email;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    SmtpClient,
    Transport,
};

// https://docs.rs/reqwest/0.10.0-alpha.2/reqwest/
// extern crate requests;
// use requests::ToJson;

use mime::TEXT_PLAIN;

use std::path::Path;

mod utils;
use utils::{
    str_from_stdin,
    default_subject_if_empty,
    default_template_if_empty,
    none_if_empty_or_some_str,
};

// use serde::{Serialize, Deserialize};

// https://github.com/lettre/lettre/issues/362
// https://github.com/lettre/lettre/issues/373.

// https://myaccount.google.com/lesssecureapps

// Refer to these if you want to write your own Python implementation instead
// 1. http://www.pybloggers.com/2018/12/sending-emails-with-python/
// 2. https://realpython.com/python-send-email/
// $pip3 install yagmail[all]

fn main() {
    // 1. Build email

    let bold = Style::new().bold();
    let yellow = Style::new().yellow();

    dotenv().ok();
    let email_author = env::var("EMAIL_AUTHOR").expect("EMAIL_AUTHOR must be set");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");

    println!("{}", bold.apply_to("Who will receive this email?"));
    let email_receiver = str_from_stdin();

    // Use requests to verify that the email was already registered
    // let body = reqwest::get("http://localhost:8000/api/email/v1/")
    //    .await?
    //    .text()
    //    .await?;

    // println!("body = {:?}", body);

    println!("{}", format!("What is the subject?{}", bold.apply_to("(Rust Developer)")));
    let subject_response = str_from_stdin();
    let email_subject = default_subject_if_empty(subject_response);

    println!("{}", format!("Which email template you want to use?{}", bold.apply_to("(resume.html)")));
    let template_response = str_from_stdin();
    let email_template = default_template_if_empty(template_response);

    let message = fs::read_to_string(Path::new(&format!("templates/{}", &email_template)))
        .expect("Something went wrong reading the file");

    println!("{}", format!("Which file you will attach with it?{}", bold.apply_to("(You can skip this)")));
    let file_response = &str_from_stdin(); // Use this just to solve lifetime problem
    let email_file = none_if_empty_or_some_str(&file_response);

    let email = match email_file {
        Some(file) => {
            Email::builder()
            .to(email_receiver.clone())
            .from(email_author.clone())
            .subject(email_subject)
            .html(message)
            .attachment_from_file(Path::new(&format!("files/{}", file)), None, &TEXT_PLAIN)
            .unwrap()
            .build()
            .unwrap()
        },
        None => {
            Email::builder()
            .to(email_receiver.clone())
            .from(email_author.clone())
            .subject(email_subject)
            .html(message)
            .build()
            .unwrap()
        },
    };

    // 2. Send it with Rust lettre

    let smtp_server = "smtp.googlemail.com";
    // https://docs.rs/lettre/0.9.2/lettre/smtp/struct.SmtpClient.html
    // Connect to a remote server on a custom port
    let mut mailer = SmtpClient::new_simple(&smtp_server)
        .unwrap()
        // Set the name sent during EHLO/HELO, default is `localhost`
        .hello_name(ClientId::Domain(smtp_server.to_string()))
        // Add credentials for authentication
        // Use API key instead later
        .credentials(Credentials::new(
            email_author.to_string(),
            email_password.to_string(),
        ))
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Enable connection reuse
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .transport();

    // 3. Verify and handle the result

    let result = mailer.send(email);

    // Use this to find what is inside
    // Ok(detail) => println!("email sent. {:#?}", detail),
    // let response = requests::get(format!("http://localhost:8000/api/email/v1/"{}, &email_receiver)).unwrap();
    // let data = response.json().unwrap();
    // println!("{:#?}", data);
    match result {
        Ok(_) => {
            // Save it to the server with a database and REST API
            // let mut map = HashMap::new();
            // map.insert("email", &email_receiver);

            // let client = reqwest::Client::new();
            // let res = client.post("http://localhost:8000/api/email/v1/")
            //    .json(&map)
            //    .send()
            //    .await?;

            println!("{}", format!(
                "The email was sent from {} to {}",
                yellow.apply_to(email_author), yellow.apply_to(email_receiver)
            ))
        },
        Err(err) => println!("Failed to send email alert: {}", err),
    }
}

// const EmailSchema = new Schema({
//   email: {
//     type: mongoose.SchemaTypes.Email,
//     required: true,
//     unique: true,
//     dropDups: true,
//   },
//   dateOfEntry: {
//     type: Date,
//     default: Date.now()
//   },
//   response: {
//     type: Boolean,
//     default: false
//   },
//   comment: {
//     type: String,
//     required: false,
//   },
// });

// #[derive(Serialize, Deserialize, Debug)]
// struct UpdateEmail {
//     response: bool,
//     comment: Option<String>,
// }

