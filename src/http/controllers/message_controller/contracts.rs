use my_http_server_swagger::*;
use serde::Serialize;

#[derive(MyHttpInput)]
pub struct PostMessageInputData {
    #[http_body(description = "Destination ip")]
    pub dest_ip: String,
    #[http_body(description = "Message")]
    pub message: String,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct PostMessageResponse {
    pub id: String,
    pub url: String,
}
#[derive(MyHttpInput)]
pub struct GetMessageInputData {
    #[http_path(description = "Id of message")]
    pub id: String,
}
