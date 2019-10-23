// Use chrono for date_of_birth and id of others?
// How to express ! for the required field?
// -> Should read Juniper documentation(https://github.com/graphql-rust/juniper)

// Juniper builds non-null types by default. A field of type Vec<Episode> will be converted into [Episode!]!. The corresponding Rust type for e.g. [Episode] would be Option<Vec<Option<Episode>>>
// -> Non null by default and if you need it, wrap it with Option

#[derive(GraphQLObject)]
#[graphql(description = "The example to compare with gRPC")]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "The example to compare with gRPC")]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}


