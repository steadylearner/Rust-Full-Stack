use tonic;
use warp;

use crate::{
    models::{
        user::{
            NewUser,
            UpdateUser,
            UserSuccess,
            UserSuccessList,
        }
    },
    user::{
        user_service_client::UserServiceClient,
        UserReply, UserRequest,
        Empty,
        Users,
        CreateUserRequest, CreateUserReply,
        UpdateUserRequest, UpdateUserReply,
        DeleteUserReply,
    },
    crypto::{
        hash::{
            hash_with_argon2,
        }
    },
};

fn create_user_success(user_reply: &UserReply) -> UserSuccess {
    let UserReply {
        id: _,
        first_name,
        last_name,
        date_of_birth: _,
    } = user_reply;

    let full_name: String = format!("{} {}", first_name, last_name);
    let hashed_full_name = hash_with_argon2(full_name.as_bytes());

    let user_success = UserSuccess {
        full_name: hashed_full_name,
    };
    user_success
}

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(Empty {});

    let response = client.unwrap().list_users(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(user) => {
            user
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };

    let Users { users } = &reply.into_inner();
    let user_success_list: Vec<UserSuccess> = users.iter().map(|user| {
        let user_success = create_user_success(user);
        user_success
    }).collect();

    // println!("{:#?}", &user_success_list);

    let user_success_list_json = UserSuccessList {
        users: user_success_list,
    };

    Ok(warp::reply::json(&user_success_list_json))
}

// Ignore error value for a while because of the typing problem
// https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
// Currently it prints error from gRPC server with println!("RESPONSE={:?}", response);
pub async fn get(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

    let response = client.unwrap().get_user(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(user) => {
            user
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };

    let user_success = create_user_success(&reply.into_inner());

    Ok(warp::reply::json(&user_success))
}

pub async fn create(create: NewUser) -> Result<impl warp::Reply, warp::Rejection> {
    // log::debug!("create_user: {:?}", create);

    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(CreateUserRequest {
        first_name: create.first_name,
        last_name: create.last_name,
        date_of_birth: create.date_of_birth,
    });

    let response = client.unwrap().create_user(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(new_user_reply) => {
            new_user_reply
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // return Err(warp::reject::custom(UserError))
            // Should return custom data not valid instead of this.
            return Err(warp::reject::not_found())
        }
    };

    let CreateUserReply {
        message,
    } = &reply.into_inner();

    // Handle type problem(Not implemented something) by making a new string with format!
    Ok(warp::reply::html(format!("{}", message)))
}

pub async fn update(id: String, update: UpdateUser) -> Result<impl warp::Reply, warp::Rejection> {
    // log::debug!("update_user: {:?}", update);

    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    // Use the default value or user data if necessary. Currently, you don't have means for that.
    // (Except sending a requet to get user data with id.)
    let request = tonic::Request::new(UpdateUserRequest {
        id,
        first_name: update.first_name,
        last_name: update.last_name,
        date_of_birth: update.date_of_birth,
    });

    let response = client.unwrap().update_user(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(update_user_reply) => {
            update_user_reply
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // return Err(warp::reject::custom(UserError))
            // Should return custom error instead of this.
            return Err(warp::reject::not_found())
        }
    };

    let UpdateUserReply {
        message,
    } = &reply.into_inner();

    // Handle type problem(Not implemented something) by making a new string with format!
    Ok(warp::reply::html(format!("{}", message)))
}

pub async fn delete(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let client = UserServiceClient::connect("http://0.0.0.0:50051").await
        .map(|client| client);

    let request = tonic::Request::new(UserRequest {
        id
    });

    let response = client.unwrap().delete_user(request).await
        .map(|response| response);

    println!("RESPONSE={:#?}", response);

    // It is required.
    let reply = match response {
        Ok(delete_user_reply) => {
            delete_user_reply
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // return Err(warp::reject::custom(UserError))
            // Should return custom unauthroized
            return Err(warp::reject::not_found())
        }
    };

    let DeleteUserReply {
        message,
    } = &reply.into_inner();

    // Handle type problem(Not implemented something) by making a new string with format!
    Ok(warp::reply::html(format!("{}", message)))
}