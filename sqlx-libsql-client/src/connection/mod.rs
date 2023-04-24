use futures_core::future::BoxFuture;
use futures_util::future;
use sqlx_core::common::StatementCache;
use sqlx_core::error::Error;
use sqlx_core::transaction::Transaction;
use std::fmt::{self, Debug, Formatter};
use std::ptr::NonNull;

use crate::{LibsqlClient, LibsqlClientConnectOptions};

pub(crate) use sqlx_core::connection::*;

mod executor;

/// A connection to an open [LibsqlClient] database.
///
/// Because SQLite is an in-process database accessed by blocking API calls, SQLx uses a background
/// thread and communicates with it via channels to allow non-blocking access to the database.
///
/// Dropping this struct will signal the worker thread to quit and close the database, though
/// if an error occurs there is no way to pass it back to the user this way.
///
/// You can explicitly call [`.close()`][Self::close] to ensure the database is closed successfully
/// or get an error otherwise.
pub struct LibsqlClientConnection {
    client: libsql_client::client::GenericClient,
}

/// Represents a callback handler that will be shared with the underlying sqlite3 connection.
pub(crate) struct Handler(NonNull<dyn FnMut() -> bool + Send + 'static>);
unsafe impl Send for Handler {}

pub(crate) struct Statements {
    // cache of semi-persistent statements
    cached: StatementCache<libsql_client::Statement>,
}

impl LibsqlClientConnection {
    pub(crate) async fn establish(options: &LibsqlClientConnectOptions) -> Result<Self, Error> {
        Ok(Self {
            client: libsql_client::new_client_from_config(libsql_client::Config {
                url: options.url.clone(),
                auth_token: options.auth_token.clone(),
            })
            .await
            .map_err(|e| Error::Protocol(e.to_string()))?,
        })
    }
}

impl Debug for LibsqlClientConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: implement this
        f.debug_struct("LibsqlClientConnection").finish()
    }
}

impl Connection for LibsqlClientConnection {
    type Database = LibsqlClient;

    type Options = LibsqlClientConnectOptions;

    fn close(mut self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move {
            drop(self);
            Ok(())
        })
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

    #[inline]
    fn shrink_buffers(&mut self) {
        // No-op.
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

impl Statements {
    fn new(capacity: usize) -> Self {
        Statements {
            cached: StatementCache::new(capacity),
        }
    }

    fn get(
        &mut self,
        query: &str,
        persistent: bool,
    ) -> Result<&mut libsql_client::Statement, Error> {
        let exists = self.cached.contains_key(query);

        if !exists {
            let statement = libsql_client::Statement::new(query);
            self.cached.insert(query, statement);
        }

        let statement = self.cached.get_mut(query).unwrap();

        Ok(statement)
    }

    fn len(&self) -> usize {
        self.cached.len()
    }

    fn clear(&mut self) {
        self.cached.clear();
    }
}
