use futures_core::future::BoxFuture;
use futures_util::future;

use crate::error::Error;
use crate::libsql_client::{LibsqlClient, LibsqlClientConnection};
use crate::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct LibsqlClientTransactionManager;

impl TransactionManager for LibsqlClientTransactionManager {
    type Database = LibsqlClient;

    fn begin(conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok(()))
    }

    fn commit(conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok(()))
    }

    fn rollback(conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement
        Box::pin(future::ok(()))
    }

    fn start_rollback(conn: &mut LibsqlClientConnection) {
        // FIXME: implement
    }
}
