use serde::{Serialize, Deserialize};
use tinydb::Database;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
struct ExampleStruct {
    my_age: i32
}

fn main() {
    let my_struct = ExampleStruct { my_age: 329 };
    let mut my_db = Database::new(String::from("query_test"), None, false);

    my_db.add_item(my_struct.clone());

    let results = my_db.query_item(|s: &ExampleStruct| &s.my_age, 329);

    assert_eq!(results.unwrap(), &my_struct);
}
