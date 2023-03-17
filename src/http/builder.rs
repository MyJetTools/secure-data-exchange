use std::sync::Arc;

use my_http_server_controllers::controllers::ControllersMiddleware;

use crate::app::AppContext;

pub async fn build_controllers(app: &Arc<AppContext>) -> ControllersMiddleware {
    let mut result = ControllersMiddleware::new(None, None);

    result.register_get_action(Arc::new(
        super::controllers::message_controller::GetAction::new(app.clone()),
    ));
    result.register_post_action(Arc::new(
        super::controllers::message_controller::PostAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::message_controller::ShowMessageAction::new(app.clone()),
    ));

    result.register_post_action(Arc::new(
        super::controllers::file_controller::PostAction::new(app.clone()),
    ));

    result.register_get_action(Arc::new(
        super::controllers::file_controller::GetAction::new(app.clone()),
    ));

    result
}
