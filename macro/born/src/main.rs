use born::{nested_macro, public_struct};

public_struct!(
    pub struct UserBase {
        pub name: String,
        pub email: String,
        pub password: String,
    }
);

UserBase!(
    #[derive(Debug)]
    pub struct User {
        pub id: i64,
    }
);

UserBase!(
    pub struct NewUser
);

fn main() {
    let new_user = NewUser {
        name: "steadylearner".into(),
        email: "example@email.com".into(),
        password: "password".into(),
    };

    let user = User {
        id: 0,
        name: new_user.name,
        email: new_user.email,
        password: new_user.password,
    };

    println!("{:#?}", &user);
}
