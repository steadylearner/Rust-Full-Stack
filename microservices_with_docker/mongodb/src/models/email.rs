use bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cat {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
    pub color: Option<String>,
    pub age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableCat {
    pub name: Option<String>,
    pub color: Option<String>,
    pub age: Option<i32>,
}

impl InsertableCat {
    fn from_cat(cats: Cat) -> InsertableCat {
        InsertableCat {
            name: cats.name,
            color: cats.color,
            age: cats.age,
        }
    }
}
