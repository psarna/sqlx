use futures_core::future::BoxFuture;

use crate::{LibsqlClient, LibsqlClientConnection};
use sqlx_core::error::Error;
use sqlx_core::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct LibsqlClientTransactionManager;

impl TransactionManager for LibsqlClientTransactionManager {
    type Database = LibsqlClient;

    fn begin(_conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement this
        Box::pin(futures_util::future::ok(()))
    }

    fn commit(_conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement this
        Box::pin(futures_util::future::ok(()))
    }

    fn rollback(_conn: &mut LibsqlClientConnection) -> BoxFuture<'_, Result<(), Error>> {
        // FIXME: implement this
        Box::pin(futures_util::future::ok(()))
    }

    fn start_rollback(_conn: &mut LibsqlClientConnection) {
        // FIXME: implement this
    }
}
