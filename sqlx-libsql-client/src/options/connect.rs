use crate::{LibsqlClientConnectOptions, LibsqlClientConnection};
use futures_core::future::BoxFuture;
use log::LevelFilter;
use sqlx_core::connection::ConnectOptions;
use sqlx_core::error::Error;
use sqlx_core::executor::Executor;
use std::fmt::Write;
use std::time::Duration;
use url::Url;

impl ConnectOptions for LibsqlClientConnectOptions {
    type Connection = LibsqlClientConnection;

    fn from_url(url: &Url) -> Result<Self, Error> {
        Self::from_url_and_token(url.path(), url.query())
    }

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized,
    {
        Box::pin(async move {
            let mut conn = LibsqlClientConnection::establish(self).await?;
            Ok(conn)
        })
    }

    fn log_statements(mut self, level: LevelFilter) -> Self {
        //self.log_settings.log_statements(level);
        self
    }

    fn log_slow_statements(mut self, level: LevelFilter, duration: Duration) -> Self {
        //self.log_settings.log_slow_statements(level, duration);
        self
    }
}
