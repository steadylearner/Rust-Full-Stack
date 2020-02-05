use warp;

use crate::{
    models::post::{
        PostList,
        Post,
        NewPost,
    },
    db::postgresql::POOL,
};

// Use this to debug and verify API chaining work or not.
// pub async fn repeat(input: String) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:#?}", &input);
//     Ok(warp::reply::html(input))
// }

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = PostList::list(&conn);
    println!("{:#?}", &response);

    Ok(warp::reply::json(&response))
}

pub async fn get(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::find(&id, &conn);

    let reply = match response {
        Ok(post) => {
            println!("{:#?}", &post);
            post
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
    Ok(warp::reply::json(&reply))
}

pub async fn create(new_post: NewPost) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = new_post
        .create(&conn);

    let reply = match response {
        Ok(new_post) => {
            println!("{:#?}", &new_post);
            new_post
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
    Ok(warp::reply::json(&reply))
}

// Make UpdatePost Struct with Optional values in it if necessary.
// Use this to make all CRUD REST API work first.
pub async fn update(id: i32, update_post: NewPost) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::update(&id, &update_post, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
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
    Ok(warp::reply::json(&reply))
}

pub async fn delete(id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Post::delete(&id, &conn);

    let reply = match response {
        Ok(null) => {
            println!("{:#?}", &null);
            null
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
    Ok(warp::reply::json(&reply))
}
