use std::sync::Arc;

use super::contracts::*;
use crate::{app::AppContext, messages::SecretMessage};
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[my_http_server_swagger::http_route(
    method: "POST",
    route: "/api/message/v1",
    summary: "Post message",
    description: "Saves message and returns it id",
    controller: "Message",
    input_data: "PostMessageInputData",
    result:[
        {status_code: 200, description: "Ok response", model: "String"},
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
    input_data: PostMessageInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let id = uuid::Uuid::new_v4().to_string();

    let mut expires = DateTimeAsMicroseconds::now();
    expires.add_minutes(5);

    let message = SecretMessage {
        id: id.clone(),
        message: input_data.message,
        expires,
        ips: vec![
            ctx.request.get_ip().get_real_ip().to_string(),
            input_data.dest_ip,
        ],
    };

    action.app.messages.add(message).await;

    return HttpOutput::as_text(id).into_ok_result(false).into();
}
