use std::sync::Arc;

use app::AppContext;

mod app;

mod http;
mod messages;

#[tokio::main]
async fn main() {
    let app = Arc::new(AppContext::new().await);

    //let mut updater_timer = MyTimer::new(Duration::from_secs(10));
    //updater_timer.register_timer("bid-ask-preview-updater", Arc::new(bid_ask_preview_timer));

    //updater_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    http::start_up::setup_server(app.clone()).await;

    app.app_states.wait_until_shutdown().await;
}
