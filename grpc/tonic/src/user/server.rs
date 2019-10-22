use tonic::{transport::Server, Request, Response, Status};

pub mod user {
    tonic::include_proto!("user");
}

use user::{
    server::{Crud, CrudServer},
    UserRequest,
    UserReply,
};

#[derive(Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl Crud for MyUser {
    async fn get_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        // request is private, so use this instead to get the data in it.
        let id = &request.into_inner().id;
        println!("Payload is {}", id);

        // Include database logic here

        let reply = if id == "It works!" {
            UserReply {
                id: "It works!".into(),
                first_name: "It works!".into(),
                last_name: "It works!".into(),
                date_of_birth: "It works!".into(),
            }
        } else {
            UserReply {
                id: "It also works!".into(),
                first_name: "It also works!".into(),
                last_name: "It also works!".into(),
                date_of_birth: "It also works!".into(),
            }
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = MyUser::default();

    Server::builder()
        .serve(addr, CrudServer::new(user))
        .await?;

    Ok(())
}

