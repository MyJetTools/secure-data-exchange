use std::sync::Arc;

use super::contracts::*;
use crate::app::AppContext;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/file/v1/{id}",
    summary: "Get file",
    description: "Returns file",
    controller: "File",
    input_data: "GetFileInputData",

    result:[
        {status_code: 200, description: "Ok response", model: "String"},
    ]
)]
pub struct GetAction {
    app: Arc<AppContext>,
}

impl GetAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &GetAction,
    input_data: GetFileInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let file = action.app.files.get(&input_data.id).await;

    if file.is_none() {
        return HttpFailResult::as_not_found("Not found".to_string(), false).into_err();
    }

    let message = file.unwrap();

    if !message.has_ip(ctx.request.get_ip().get_real_ip()) {
        return HttpFailResult::as_not_found("Not found".to_string(), false).into_err();
    }

    return HttpOutput::File {
        file_name: message.file.file_name.clone(),
        content: message.file.content.clone(),
    }
    .into_ok_result(false)
    .into();
}
