use std::sync::Arc;

use reqwest::Client;

use crate::db::DbPool;

pub(crate) struct AppConfig {
    pub client: Arc<Client>,
    pub pool: DbPool,
}
