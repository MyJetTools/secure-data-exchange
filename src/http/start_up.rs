use std::{net::SocketAddr, sync::Arc};

use my_http_server::{MyHttpServer, StaticFilesMiddleware};
use my_http_server_controllers::swagger::SwaggerMiddleware;

use crate::app::AppContext;

pub async fn setup_server(app: Arc<AppContext>) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    let controllers = Arc::new(super::builder::build_controllers(&app).await);

    http_server.add_middleware(Arc::new(StaticFilesMiddleware::new(
        None,
        Some(vec!["/index.html".to_string()]),
    )));

    let swagger_middleware = SwaggerMiddleware::new(
        controllers.clone(),
        crate::app::APP_NAME.to_string(),
        crate::app::APP_VERSION.to_string(),
    );

    http_server.add_middleware(Arc::new(swagger_middleware));

    http_server.add_middleware(controllers);

    http_server.start(app.app_states.clone(), my_logger::LOGGER.clone());
}
