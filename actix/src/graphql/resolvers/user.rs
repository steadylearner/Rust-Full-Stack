//  Read https://graphql-rust.github.io/juniper/current/schema/schemas_and_mutations.html
//  and compare to this file.

use juniper::FieldResult;
use juniper::RootNode;

use crate::type_defs::user::{User, NewUser};

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field user(&executor, id: String) -> FieldResult<User> {
        Ok(User{
            id: "It works!".to_owned(),
            first_name: "It works!".to_owned(),
            last_name: "It works!".to_owned(),
            date_of_birth: "It works!".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            id: "It also works!".to_owned(),
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            date_of_birth: new_user.date_of_birth,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
