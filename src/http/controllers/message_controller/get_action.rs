use std::sync::Arc;

use super::contracts::*;
use crate::app::AppContext;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/message/v1/{id}",
    summary: "Get message",
    description: "Returns message",
    controller: "Message",
    input_data: "GetMessageInputData",

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
    input_data: GetMessageInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let message = action.app.messages.get(&input_data.id).await;

    if message.is_none() {
        return HttpFailResult::as_not_found("Not found".to_string(), false).into_err();
    }

    let message = message.unwrap();

    if !message.has_ip(ctx.request.get_ip().get_real_ip()) {
        return HttpFailResult::as_not_found("Not found".to_string(), false).into_err();
    }

    return HttpOutput::as_text(message.message.to_string())
        .into_ok_result(false)
        .into();
}
