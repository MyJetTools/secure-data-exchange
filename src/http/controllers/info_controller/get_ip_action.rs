use std::sync::Arc;

use crate::app::AppContext;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/ip",
    summary: "Get ip",
    description: "Returns Ip",
    controller: "Info",


    result:[
        {status_code: 200, description: "Ok response", model: "String"},
    ]
)]
pub struct GetIpAction {
    _app: Arc<AppContext>,
}

impl GetIpAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { _app: app }
    }
}
async fn handle_request(
    _action: &GetIpAction,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let ip = ctx.request.get_ip();
    return HttpOutput::as_text(ip.get_real_ip_as_string())
        .into_ok_result(false)
        .into();
}
