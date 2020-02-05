use serde_derive::{Deserialize, Serialize};

// I want to use this but there are many type relevant problems with this.
// use chrono::NaiveDate;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct NewUser {
//     pub first_name: String,
//     pub last_name: String,
//     pub date_of_birth: NaiveDate,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

// It works when every fields is defined. But, there will be a better solution than this.
// The simplest one will be send get request for the id first and compare the values.
// Should edit gRPC server and this at the same time?
// https://www.google.com/search?q=what+does+COALESCE+in+postgresql
// https://stackoverflow.com/questions/13305878/dont-update-column-if-update-value-is-null
// UPDATE some_table SET
//     column_1 = COALESCE(param_1, column_1),
//     column_2 = COALESCE(param_2, column_2),
//     column_3 = COALESCE(param_3, column_3),
//     column_4 = COALESCE(param_4, column_4),
//     column_5 = COALESCE(param_5, column_5)
// WHERE id = some_id;

// Otherwise, write client side code for this and use existing values from a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
}

// Use this instead of the code below
// https://github.com/serde-rs/json#constructing-json-values
// let user_success = json!({
//     "full_name": &hashed_full_name,
// });
#[derive(Serialize, Deserialize, Debug)]
pub struct UserSuccess {
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSuccessList {
    pub users: Vec<UserSuccess> // or Vec<Option<UseSuccess>>?
}

// // I don't need it at this point.
// // "success" should be always false for this struct. Find the better design.
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UserError {
//     pub error: String,
//     pub success: bool,
// }