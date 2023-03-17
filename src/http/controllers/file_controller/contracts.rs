use my_http_server::types::FileContent;
use my_http_server_swagger::*;
use serde::Serialize;

#[derive(MyHttpInput)]
pub struct PostFileInputData {
    #[http_form_data(description = "Destination ip")]
    pub dest_ip: String,
    #[http_form_data(description = "File")]
    pub file: FileContent,
}

#[derive(MyHttpObjectStructure, Serialize)]
pub struct PostFileResponse {
    pub id: String,
    pub url: String,
}

#[derive(MyHttpInput)]
pub struct GetFileInputData {
    #[http_path(description = "Id of file")]
    pub id: String,
}
