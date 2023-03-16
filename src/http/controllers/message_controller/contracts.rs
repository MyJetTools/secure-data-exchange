use my_http_server_swagger::*;

#[derive(MyHttpInput)]
pub struct PostMessageInputData {
    #[http_query(description = "Destination ip")]
    pub dest_ip: String,
    #[http_query(description = "Message")]
    pub message: String,
}

#[derive(MyHttpInput)]
pub struct GetMessageInputData {
    #[http_path(description = "Id of message")]
    pub id: String,
}
