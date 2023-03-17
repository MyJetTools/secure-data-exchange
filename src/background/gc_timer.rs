use std::sync::Arc;

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::app::AppContext;

pub struct GcTimer {
    app: Arc<AppContext>,
}

impl GcTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl MyTimerTick for GcTimer {
    async fn tick(&self) {
        let now = DateTimeAsMicroseconds::now();
        self.app.messages.gc(now).await;
        self.app.files.gc(now).await;
    }
}
