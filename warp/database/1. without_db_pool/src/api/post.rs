// Make data first with post.sql
// Use macros to ignore type problems and make them sharable to tests/
#[macro_export]
macro_rules! list_posts {
    () => {
        // $curl 0.0.0.0:8000/api/post/v1
        post_route::list()
            .and_then(post_handler::list)
    }
}

#[macro_export]
macro_rules! get_post {
    () => {
        // $curl 0.0.0.0:8000/api/post/v1/1
        post_route::get()
            .and_then(post_handler::get)
    }
}

#[macro_export]
macro_rules! create_post {
    () => {
        // curl -X POST localhost:8000/api/post/v1 -H "Content-Type: application/json" -d '{ "title": "When can I work with programming?", "body": "Should find someone recognize my skills." }'
        // and $curl 0.0.0.0:8000/api/post/v1
        // To see it created.
        post_route::create()
            .and_then(post_handler::create)
    }
}

#[macro_export]
macro_rules! update_post {
    () => {
        // curl -X PUT 0.0.0.0:8000/api/post/v1/1 -H "Content-Type: application/json" -d '{ "title": "test update", "body": "test update" }'
        // and $curl 0.0.0.0:8000/api/post/v1/1
        // To see it updated.
        post_route::update()
            .and_then(post_handler::update)
    }
}

#[macro_export]
macro_rules! delete_post {
    () => {
        // $curl -X DELETE 0.0.0.0:8000/api/post/v1/1 and $curl 0.0.0.0:8000/api/post/v1
        // To see it deleted.
        post_route::delete()
            .and_then(post_handler::delete)
    }
}