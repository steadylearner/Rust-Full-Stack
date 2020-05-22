use born::{
    nested_macro,
    public_struct,
};

public_struct!(
    pub struct UserBase {
        pub first_name: String,
        pub last_name: String,
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
        first_name: "steady".into(),
        last_name: "learner".into(),
        email: "example@email.com".into(),
        password: "password".into(),
    };

    let user = User {
        id: 0,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        email: new_user.email,
        password: new_user.password,
    };

    println!("{:#?}", &user);
}
