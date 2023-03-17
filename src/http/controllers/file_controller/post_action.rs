use std::sync::Arc;

use super::contracts::*;
use crate::{app::AppContext, files::SecretFile};
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/file/v1",
    summary: "Post file",
    description: "Saves file and returns it id",
    controller: "File",
    input_data: "PostFileInputData",
    result:[
        {status_code: 200, description: "Ok response", model: "PostFileResponse"},
    ]
)]
pub struct PostAction {
    app: Arc<AppContext>,
}

impl PostAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &PostAction,
    input_data: PostFileInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let id = uuid::Uuid::new_v4().to_string();

    let mut expires = DateTimeAsMicroseconds::now();
    expires.add_minutes(5);

    let file = SecretFile {
        id: id.clone(),
        file: input_data.file,
        expires,
        ips: vec![
            ctx.request.get_ip().get_real_ip().to_string(),
            input_data.dest_ip,
        ],
    };

    action.app.files.add(file).await;

    let url = format!(
        "{}://{}/file/{}",
        ctx.request.get_scheme(),
        ctx.request.get_host(),
        id
    );

    return HttpOutput::as_json(PostFileResponse { id, url })
        .into_ok_result(false)
        .into();
}
