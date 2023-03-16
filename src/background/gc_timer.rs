use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

use rust_extensions::{date_time::DateTimeAsMicroseconds, MyTimerTick};

use crate::{
    app::AppContext,
    my_no_sql::{BidAskPreviewMyNoSqlEntity, PREVIEW_UPDATE_INTERVAL},
};

pub struct BidAskPreviewUpdaterTimer {
    app: Arc<AppContext>,
    last_updated: AtomicI64,
}

impl BidAskPreviewUpdaterTimer {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self {
            app,
            last_updated: AtomicI64::new(0),
        }
    }

    async fn get_last_updated_date_time(&self) -> DateTimeAsMicroseconds {
        let mut last_updated = self.last_updated.load(std::sync::atomic::Ordering::Relaxed);

        if last_updated > 0 {
            return DateTimeAsMicroseconds::new(last_updated);
        }

        let entities = self
            .app
            .bid_ask_preview_writer
            .get_by_partition_key(BidAskPreviewMyNoSqlEntity::generate_partition_key(), None)
            .await
            .unwrap();

        last_updated = 1;

        if entities.is_none() {
            self.last_updated
                .store(last_updated, std::sync::atomic::Ordering::Relaxed);
            return DateTimeAsMicroseconds::new(last_updated);
        }

        for entity in entities.unwrap() {
            if entity.last_updated > last_updated {
                last_updated = entity.last_updated;
            }
        }

        self.last_updated
            .store(last_updated, std::sync::atomic::Ordering::Relaxed);
        DateTimeAsMicroseconds::new(last_updated)
    }
}

#[async_trait::async_trait]
impl MyTimerTick for BidAskPreviewUpdaterTimer {
    async fn tick(&self) {
        let last_updated = self.get_last_updated_date_time().await;

        let now = DateTimeAsMicroseconds::now();

        if now.duration_since(last_updated).as_positive_or_zero() < PREVIEW_UPDATE_INTERVAL {
            return;
        }

        let bid_asks = self
            .app
            .bid_ask_snapshot_reader
            .get_entities(BidAskSnapshotNoSqlEntity::generate_partition_key())
            .get_as_vec()
            .await;

        if bid_asks.is_none() {
            println!("No bid asks - skipping the iteration");
            return;
        }

        let entities = self
            .app
            .bid_ask_preview_writer
            .get_by_partition_key(BidAskPreviewMyNoSqlEntity::generate_partition_key(), None)
            .await
            .unwrap();

        let mut entities_by_hash_map = HashMap::new();

        if let Some(entities) = entities {
            for entity in entities {
                entities_by_hash_map.insert(entity.row_key.clone(), entity);
            }
        }

        for bid_ask in bid_asks.unwrap() {
            if let Some(entity) = entities_by_hash_map.get_mut(&bid_ask.row_key) {
                entity.update_bid(bid_ask.bid, now);
            } else {
                entities_by_hash_map.insert(
                    bid_ask.row_key.clone(),
                    BidAskPreviewMyNoSqlEntity::new(&bid_ask.row_key, bid_ask.bid, now),
                );
            }
        }

        let mut entities_as_vec = Vec::with_capacity(entities_by_hash_map.len());

        for (_, entity) in entities_by_hash_map {
            entities_as_vec.push(entity);
        }

        self.app
            .bid_ask_preview_writer
            .bulk_insert_or_replace(&entities_as_vec)
            .await
            .unwrap();

        self.last_updated
            .store(now.unix_microseconds, std::sync::atomic::Ordering::SeqCst);
    }
}
