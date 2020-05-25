#[macro_use] extern crate born_attribute;

#[no_field]
#[derive(Debug)]
struct NoField;

#[user_base]
#[derive(Debug)]
struct UserCreateRequest {
    name: String,
}

#[message_base]
#[derive(Debug)]
struct MessageCreateRequest {
    from: String,
    to: String
}

fn main() {
    let no_field = NoField {
        useful: false,
    };
    println!("{:#?}", no_field);

    let user = UserCreateRequest {
        name: "www.steadylearner.com".into(),
        active: true,
    };

    let message_create_request = MessageCreateRequest {
        text: "You can do the same with functional macros from the crate I will publish.".into(),
        from: user.name,
        to: "Rust developers".into(),
    };
    println!("{:#?}", &message_create_request);
}
