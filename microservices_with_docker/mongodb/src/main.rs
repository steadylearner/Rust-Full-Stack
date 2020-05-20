// The API we will use mostly.
// https://docs.rs/mongodb/0.9.0/mongodb/struct.Collection.html
// Or, $cargo doc -p mongodb --open

// Refer to this example.
// https://medium.com/@louis.beaumont/rest-api-with-rust-mongodb-10eeb6bd51d7

extern crate mongodb;
use mongodb::{
    Client,
    // The authors should think how to make easily startable examples.
    // options::ClientOptions
};

#[macro_use]
extern crate bson;
use bson::doc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb://0.0.0.0:27017")?;

    let db = client.database("emails");
    for coll_name in db.list_collection_names(None)? {
        println!("collection: {}", coll_name);
    }

    let coll = db.collection("emails");
    // Where is doc! macro?, Not in documenation API.
    let result = coll.insert_one(doc! { "email": "steady@learner.com", "response": false,}, None)?;
    println!("{:#?}", result);
    // InsertOneResult {
    //     inserted_id: ObjectId(ObjectId(5e0a4c87009c794000b522b7)),
    // }

    // In mongob shell
    // db.emails.find()

    Ok(())
}

// [JavaScript] -> [Rust]
// 1. Create email

// $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
// // $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
// const registerEmail = async (req, res) => {
//     const newEmail = new Email({
//         email: req.body.email,
//     });

//     try {
//         const newItem = await newEmail.save();
//         res.json(newItem);
//     } catch(e) {
//         res.json({
//             ...e,
//             success: false,
//         });

//         console.log(e);
//     }
// };

// 2. Read email

// curl -X GET localhost:8000/api/email/v1/steady@learner.com
// const getEmail = async (req, res) => {
//     try {
//         const item = Email.findOne({
//             email: req.params.email
//         });
//         console.log(res.json(items));
//     } catch (e) {
//         res.json({
//             ...e,
//             success: false,
//         });
//         console.log(e);
//     }
// };

// 3. Delete email

// $curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
// const deleteEmail = async (req, res) => {
//     try {
//         const _deletedItem = await Email.findOneAndDelete({ email: req.params.email });
//         res.json({ success: true });
//     } catch (e) {
//         console.log(e);
//         res.status(404).json({ success: false });
//     }
// };

// 4. Update email

// $curl -X PUT localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
// const updateEmail = async (req, res) => {
//     try {
//         const _updatedItem = await Email.findOneAndUpdate({ email: req.params.email }, req.body);
//         res.json({ success: true });
//     } catch(e) {
//         console.log(e);
//         res.status(404).json({ success: false });
//     }
// };

// 5. List email

// $curl -X GET localhost:8000/api/email/v1
// const listEmails = async (_req, res) => {
//     try {
//         const items = await Email.find().sort({ date: -1 });
//         console.log(res.json(items));
//     } catch(e) {
//         res.json({
//             ...e,
//             success: false,
//         });
//         console.log(e);
//     }
// };
