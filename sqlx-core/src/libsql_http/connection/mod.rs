use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ptr::NonNull;

use futures_core::future::BoxFuture;
use futures_intrusive::sync::MutexGuard;
use futures_util::future;

pub(crate) use handle::{ConnectionHandle, ConnectionHandleRaw};

use crate::common::StatementCache;
use crate::connection::{Connection, LogSettings};
use crate::error::Error;
use crate::libsql_http::connection::establish::EstablishParams;
use crate::libsql_http::statement::VirtualStatement;
use crate::libsql_http::{LibsqlHttp, LibsqlHttpConnectOptions};
use crate::transaction::Transaction;

pub(crate) mod describe;
pub(crate) mod establish;
pub(crate) mod execute;
mod executor;
mod explain;
mod handle;

/// A connection to an open [LibsqlHttp] database.
pub struct LibsqlHttpConnection {
    pub(crate) session: libsql_client::Session,
}

pub struct LockedLibsqlHttpHandle<'a> {
    pub(crate) guard: MutexGuard<'a, ConnectionState>,
}

pub(crate) struct ConnectionState {
    pub(crate) handle: ConnectionHandle,

    // transaction status
    pub(crate) transaction_depth: usize,

    pub(crate) statements: Statements,

    log_settings: LogSettings,
}

pub(crate) struct Statements {
    // cache of semi-persistent statements
    cached: StatementCache<VirtualStatement>,
    // most recent non-persistent statement
    temp: Option<VirtualStatement>,
}

impl LibsqlHttpConnection {
    pub(crate) async fn establish(options: &LibsqlHttpConnectOptions) -> Result<Self, Error> {
        let client = libsql_client::Session::connect_from_ctx(&options.ctx)?;
        Ok(Self {
            client
        })
    }
}

impl Debug for LibsqlHttpConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LibsqlHttpConnection").finish()
    }
}

impl Connection for LibsqlHttpConnection {
    type Database = LibsqlHttp;

    type Options = LibsqlHttpConnectOptions;

    fn close(mut self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(future::ok(()))
    }

    fn close_hard(self) -> BoxFuture<'static, Result<(), Error>> {
        Box::pin(async move {
            drop(self);
            Ok(())
        })
    }

    /// Ensure the background worker thread is alive and accepting commands.
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

impl Drop for ConnectionState {
    fn drop(&mut self) {
        // explicitly drop statements before the connection handle is dropped
        self.statements.clear();
    }
}

impl Statements {
    fn new(capacity: usize) -> Self {
        Statements {
            cached: StatementCache::new(capacity),
            temp: None,
        }
    }

    fn get(&mut self, query: &str, persistent: bool) -> Result<&mut VirtualStatement, Error> {
        if !persistent || !self.cached.is_enabled() {
            return Ok(self.temp.insert(VirtualStatement::new(query, false)?));
        }

        let exists = self.cached.contains_key(query);

        if !exists {
            let statement = VirtualStatement::new(query, true)?;
            self.cached.insert(query, statement);
        }

        let statement = self.cached.get_mut(query).unwrap();

        if exists {
            // as this statement has been executed before, we reset before continuing
            statement.reset()?;
        }

        Ok(statement)
    }

    fn len(&self) -> usize {
        self.cached.len()
    }

    fn clear(&mut self) {
        self.cached.clear();
        self.temp = None;
    }
}
