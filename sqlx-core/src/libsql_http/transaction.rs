use futures_core::future::BoxFuture;

use crate::error::Error;
use crate::sqlite::{LibsqlHttp, LibsqlHttpConnection};
use crate::transaction::TransactionManager;

/// Implementation of [`TransactionManager`] for SQLite.
pub struct LibsqlHttpTransactionManager;

impl TransactionManager for LibsqlHttpTransactionManager {
    type Database = LibsqlHttp;

    fn begin(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.begin())
    }

    fn commit(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.commit())
    }

    fn rollback(conn: &mut LibsqlHttpConnection) -> BoxFuture<'_, Result<(), Error>> {
        Box::pin(conn.worker.rollback())
    }

    fn start_rollback(conn: &mut LibsqlHttpConnection) {
        conn.worker.start_rollback().ok();
    }
}
