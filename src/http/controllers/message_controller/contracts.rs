use my_http_server_swagger::*;

#[derive(MyHttpInput)]
pub struct PostMessageInputData {
    #[http_body(description = "Destination ip")]
    pub dest_ip: String,
    #[http_body(description = "Message")]
    pub message: String,
}

#[derive(MyHttpInput)]
pub struct GetMessageInputData {
    #[http_path(description = "Id of message")]
    pub id: String,
}
