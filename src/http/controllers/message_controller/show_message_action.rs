use std::sync::Arc;

use super::contracts::*;
use crate::app::AppContext;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput, WebContentType};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/message/{id}",
    input_data: "GetMessageInputData",
)]
pub struct ShowMessageAction {
    app: Arc<AppContext>,
}

impl ShowMessageAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}
async fn handle_request(
    action: &ShowMessageAction,
    input_data: GetMessageInputData,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let message = action.app.messages.get(&input_data.id).await;

    if message.is_none() {
        return HttpFailResult::as_not_found("Page Not found".to_string(), false).into_err();
    }

    let message = message.unwrap();

    if !message.has_ip(ctx.request.get_ip().get_real_ip()) {
        return HttpFailResult::as_not_found("Page Not found".to_string(), false).into_err();
    }

    return HttpOutput::Content {
        content: get_html(&message.message).into_bytes(),
        headers: None,
        content_type: WebContentType::Html.into(),
    }
    .into_ok_result(false)
    .into();
}

fn get_html(message: &str) -> String {
    format!(
        r#"
    <!DOCTYPE html>
 <html>
 
 <head>
     <meta charset="utf-8">
     <title>Super Chat</title>
     <base href="/">
     <link href="/css/bootstrap.css" rel="stylesheet" type="text/css" />
     <script src="/js/jquery.js"></script>
     <meta name="viewport" content="width=device-width, initial-scale=1">
     <link rel="icon" type="image/x-icon" href="favicon.ico">
 </head>
 
 <body>
 <div style="margin:10px">
  {} 
</div>
 </body>
    "#,
        message
    )
}
