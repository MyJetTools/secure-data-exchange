use std::{sync::Arc, time::Duration};

use app::AppContext;
use rust_extensions::MyTimer;

mod app;

mod background;
mod files;
mod http;
mod messages;

#[tokio::main]
async fn main() {
    let app = Arc::new(AppContext::new().await);

    let mut gc_timer = MyTimer::new(Duration::from_secs(10));
    gc_timer.register_timer("gc", Arc::new(crate::background::GcTimer::new(app.clone())));

    gc_timer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    http::start_up::setup_server(app.clone()).await;

    app.app_states.wait_until_shutdown().await;
}
