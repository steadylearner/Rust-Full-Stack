extern crate console;
extern crate dotenv;
extern crate lettre;
extern crate uuid;
use dotenv::dotenv;
use std::env;

use console::Style;
use std::fs;
use std::io::stdin;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::{
    EmailAddress,
    Envelope,
    SendableEmail,
    SmtpClient,
    Transport
};

use uuid::Uuid;

fn str_from_stdin() -> String {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    // Remove \n
    let user_input = user_input[..(user_input.len() - 1)].to_string();

    user_input
}

// https://github.com/lettre/lettre/issues/362
// https://github.com/lettre/lettre/issues/373.

// I can write a better code with Rust than Python because I used it more. But API should be organized.

fn main() {
    // 1. Build email

    let bold = Style::new().bold();
    let yellow = Style::new().yellow();

    dotenv().ok();
    let email_author = env::var("EMAIL_AUTHOR").expect("EMAIL_AUTHOR must be set");
    let email_password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set");

    println!("{}", bold.apply_to("Who will receive this email?"));
    let email_receiver = str_from_stdin();
    let email_id = Uuid::new_v4();

    println!("{}", bold.apply_to("Which email template you want to use?"));
    let email_template = str_from_stdin();

    let message = fs::read(format!("templates/{}", &email_template))
        .expect("Something went wrong reading the file");

    // https://docs.rs/lettre/0.9.2/lettre/struct.SendableEmail.html
    let email = SendableEmail::new(
        // https://docs.rs/lettre/0.9.2/lettre/struct.Envelope.html
        Envelope::new(
            Some(EmailAddress::new(email_author.to_string()).unwrap()),
            vec![EmailAddress::new(email_receiver.to_string()).unwrap()],
        )
        .unwrap(),
        email_id.to_string(), // Read more documenation, use uuid or whatever here and save to a database etc?
        message,              // How can I send html?
    );

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
    match result {
        Ok(_) => println!("{}", format!(
            "The email was sent from {} to {}",
            yellow.apply_to(email_author), yellow.apply_to(email_receiver)
        )),
        Err(err) => println!("Failed to send email alert: {}", err),
    }
}
