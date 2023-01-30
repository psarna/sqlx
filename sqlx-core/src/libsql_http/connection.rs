use crate::connection::{ConnectOptions, Connection};
use futures_core::future::BoxFuture;
use futures_util::future;
use libsql_client::Session;
use log::LevelFilter;
use std::time::Duration;

use crate::error::Error;
use crate::libsql_http::Libsql;
use crate::transaction::Transaction;

pub struct LibsqlHttpConnection {
    session: Session,
}

impl Connection for LibsqlHttpConnection {
    type Database = Libsql;
    type Options = LibsqlHttpConnectOptions;

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
        Transaction::begin(self)
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
pub struct LibsqlHttpConnectOptions {
    url: String,
    user: String,
    pass: String,
}

impl std::str::FromStr for LibsqlHttpConnectOptions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME: actually parse the information from uri
        Ok(Self {
            url: String::new(),
            user: String::new(),
            pass: String::new(),
        })
    }
}

impl ConnectOptions for LibsqlHttpConnectOptions {
    type Connection = LibsqlHttpConnection;

    /// Establish a new database connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, Error>>
    where
        Self::Connection: Sized,
    {
        Box::pin(future::ok(Self::Connection {
            session: libsql_client::Session::connect(self.url, self.user, self.pass),
        }))
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
