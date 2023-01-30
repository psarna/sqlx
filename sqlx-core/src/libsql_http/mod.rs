//! **libSQL** http driver.

pub use arguments::{LibsqlHttpArgumentValue, LibsqlHttpArguments};
pub use column::LibsqlHttpColumn;
pub use connection::{LockedLibsqlHttpHandle, LibsqlHttpConnection};
pub use database::LibsqlHttp;
pub use error::LibsqlHttpError;
pub use options::{
    LibsqlHttpAutoVacuum, LibsqlHttpConnectOptions, LibsqlHttpJournalMode, LibsqlHttpLockingMode, LibsqlHttpSynchronous,
};
pub use query_result::LibsqlHttpQueryResult;
pub use row::LibsqlHttpRow;
pub use statement::LibsqlHttpStatement;
use std::sync::atomic::AtomicBool;
pub use transaction::LibsqlHttpTransactionManager;
pub use type_info::LibsqlHttpTypeInfo;
pub use value::{LibsqlHttpValue, LibsqlHttpValueRef};

use crate::describe::Describe;
use crate::error::Error;
use crate::executor::Executor;
use crate::sqlite::connection::establish::EstablishParams;

mod arguments;
mod column;
mod connection;
mod database;
mod error;
mod options;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_info;
pub mod types;
mod value;

/// An alias for [`Executor<'_, Database = LibsqlHttp>`][Executor].
pub trait LibsqlHttpExecutor<'c>: Executor<'c, Database = LibsqlHttp> {}
impl<'c, T: Executor<'c, Database = LibsqlHttp>> LibsqlHttpExecutor<'c> for T {}

// NOTE: required due to the lack of lazy normalization
impl_into_arguments_for_arguments!(LibsqlHttpArguments<'q>);
impl_executor_for_pool_connection!(LibsqlHttp, LibsqlHttpConnection, LibsqlHttpRow);
impl_executor_for_transaction!(LibsqlHttp, LibsqlHttpRow);
impl_column_index_for_row!(LibsqlHttpRow);
impl_column_index_for_statement!(LibsqlHttpStatement);
impl_acquire!(LibsqlHttp, LibsqlHttpConnection);
impl_into_maybe_pool!(LibsqlHttp, LibsqlHttpConnection);

// required because some databases have a different handling of NULL
impl_encode_for_option!(LibsqlHttp);
