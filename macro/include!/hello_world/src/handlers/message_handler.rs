use warp::{
    Reply, Rejection,
    reply,
};

use crate::{
    models::{
        MessageCreateRequest,
        MessageCreateReply,
    }
};

pub async fn message(message_create_request: MessageCreateRequest) -> Result<impl Reply, Rejection> {
    let MessageCreateRequest { text } = message_create_request;
    let reply = MessageCreateReply {
        text,
    };

    Ok(reply::json(&reply))
}
