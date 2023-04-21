use crate::connection::{ConnectOptions, Connection};
use futures_core::future::BoxFuture;
use futures_util::future;
use log::LevelFilter;
use std::time::Duration;

use crate::error::Error;
use crate::libsql_client::LibsqlClient;
use crate::transaction::Transaction;

pub struct LibsqlClientConnection {
    client: libsql_client::client::GenericClient,
}

// FIXME: add details, or just implement Debug for GenericClient
impl std::fmt::Debug for LibsqlClientConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LibsqlClientConnection").finish()
    }
}

impl Connection for LibsqlClientConnection {
    type Database = LibsqlClient;
    type Options = LibsqlClientConnectOptions;

    fn close(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn ping(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn begin(&mut self) -> BoxFuture<'_, Result<Transaction<'_, Self::Database>, Error>>
    where
        Self: Sized,
    {
        panic!();
    }

    fn cached_statements_size(&self) -> usize {
        0
    }

    fn clear_cached_statements(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    #[doc(hidden)]
    fn flush(&mut self) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    #[doc(hidden)]
    fn should_flush(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct LibsqlClientConnectOptions {
    pub url: String,
    pub auth_token: String,
}

impl std::str::FromStr for LibsqlClientConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME: actually parse the information from uri
        Ok(Self {
            url: String::new(),
            auth_token: String::new(),
        })
    }
}

impl ConnectOptions for LibsqlClientConnectOptions {
    type Connection = LibsqlClientConnection;

    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized,
    {
        use futures_util::TryFutureExt;
        let url = match url::Url::parse(&self.url) {
            Ok(url) => url,
            Err(err) => return Box::pin(future::err(Error::Protocol(err.to_string()))),
        };
        let auth_token = Some(self.auth_token.clone());
        let client_fut = async move {
            match libsql_client::new_client_from_config(libsql_client::Config { url, auth_token })
                .await
            {
                Ok(client) => Ok(client),
                Err(err) => Err(Error::Protocol(err.to_string())),
            }
        };
        let connect_fut =
            client_fut.and_then(|client| async move { Ok(Self::Connection { client }) });
        Box::pin(connect_fut)
    }

    /// Log executed statements with the specified `level`
    fn log_statements(&mut self, level: LevelFilter) -> &mut Self {
        self
    }

    /// Log executed statements with a duration above the specified `duration`
    /// at the specified `level`.
    fn log_slow_statements(&mut self, level: LevelFilter, duration: Duration) -> &mut Self {
        self
    }
}
