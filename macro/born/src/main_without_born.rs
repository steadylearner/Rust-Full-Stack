#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

fn main() {
    let new_user = NewUser {
        name: "steadylearner".into(),
        email: "example@email.com".into(),
        password: "password".into(),
    };

    let user = User {
        id: 0,
        name: new_user.first_name,
        email: new_user.email,
        password: new_user.password,
    };

    println!("{:#?}", &user);
}
