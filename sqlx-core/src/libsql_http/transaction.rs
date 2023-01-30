use futures_core::future::BoxFuture;
use futures_util::future;

use crate::error::Error;
use crate::libsql_http::LibsqlHttpConnection;
use crate::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct LibsqlHttpTransactionManager;

impl TransactionManager for LibsqlHttpTransactionManager {
    type Database = i32; // fixme: super::Database

    fn begin(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok())
    }

    fn commit(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok())
    }

    fn rollback(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok())
    }

    fn start_rollback(conn: &mut LibsqlHttpConnection) {
        // FIXME: implement
    }
}
