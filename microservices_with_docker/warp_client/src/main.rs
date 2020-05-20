extern crate pretty_env_logger;
#[macro_use] extern crate log;
// This is to test files in benches/
extern crate reqwest;

extern crate tonic;
mod user {
    tonic::include_proto!("user");
}

extern crate argon2;
extern crate rand;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate warp;
use warp::{
    Filter,
};

extern crate console;
use console::Style;

// This is where all your custom modules(folders) will be.
mod crypto;
mod models;
mod routes;
// Files in handlers/ are what implements "Result<impl warp::Reply, warp::Rejection>"
// It will be similar to controllers in Express and you will edit it most of time with models/
mod handlers;
use self::{
    routes::{
        user_route,
    },
    handlers::{
        user_handler
    },

};

// // It will only work with $cargo bench
// #[cfg(bench)] mod benches;

// It will only work with $cargo test
#[cfg(test)] mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // You may want to extract these to functions but you will get private type errors.

    // curl 0.0.0.0:8000/api/user/v1
    let list_users = user_route::list()
        .and_then(user_handler::list);

    // curl 0.0.0.0:8000/api/user/v1/steadylearner
    let get_user = user_route::get()
        .and_then(user_handler::get);

    // curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json" -d '{ "first_name": "steady", "last_name": "learner", "date_of_birth": "2019-01-01" }'
    let create_user = user_route::create()
        .and_then(user_handler::create);

    // curl -X PUT 0.0.0.0:8000/api/user/v1/steadylearner -H "authorization: user" -H "Content-Type: application/json" -d '{ "first_name": "fullstack rust", "last_name": "developer", "date_of_birth": "2019-01-01" }'
    // It becomes {
    //     id: "steadylearner",
    //     first_name: "fullstack rust",
    //     last_name: "developer",
    //     date_of_birth: "2019-01-01",
    // },
    // curl -X PUT 0.0.0.0:8000/api/user/v1/steadylearner -H "authorization: user" -H "Content-Type: application/json" -d '{ "first_name": "steady", "last_name": "learner", "date_of_birth": "2019-01-01" }'
    // It becomes {
    //     id: "steadylearner",
    //     first_name: "steady",
    //     last_name: "learner",
    //     date_of_birth: "2019-01-01",
    // },
    let update_user = user_route::update()
        .and_then(user_handler::update);

    // curl -X DELETE -H "authorization: steadylearner" 0.0.0.0:8000/api/user/v1/f2bd8139-5044-4526-89b8-1981d6220b4
    // No more records in Postgresql.
    // \c grpc;
    // $SELECT * FROM users WHERE id = 'f2bd8139-5044-4526-89b8-1981d6220b4';
    let delete_user = user_route::delete()
        .and_then(user_handler::delete);

    let api = list_users
        .or(get_user)
        .or(create_user)
        .or(update_user)
        .or(delete_user);

    // The complete form should be this with tests
    // let api = list_users.or(get_user).or(create_user).or(update_user).or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// 1. Read the Warp documentation more for Response when there are errors
// 2. Use Redis to cache data for get_user and list_users and remove the data when update_user, delete_user.
// 3. Why Browser and curl returns 405 Method not allowed? While the code should return return 404
//    and test for it pass  Not Found for get_user with wrong id?
