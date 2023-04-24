//! **libSQL client** database driver.

#[macro_use]
extern crate sqlx_core;

pub use arguments::LibsqlClientArguments;
pub use column::LibsqlClientColumn;
pub use connection::LibsqlClientConnection;
pub use database::LibsqlClient;
pub use error::LibsqlClientError;
pub use options::LibsqlClientConnectOptions;
pub use query_result::LibsqlClientQueryResult;
pub use row::LibsqlClientRow;
pub use statement::LibsqlClientStatement;
pub use transaction::LibsqlClientTransactionManager;
pub use type_info::LibsqlClientTypeInfo;
pub use value::{LibsqlClientValue, LibsqlClientValueRef};

pub(crate) use sqlx_core::driver_prelude::*;

use sqlx_core::executor::Executor;

mod arguments;
mod column;
mod connection;
mod database;
mod error;
mod logger;
mod options;
mod query_result;
mod row;
mod statement;
mod transaction;
mod type_info;
pub mod types;
mod value;

#[cfg(feature = "any")]
pub mod any;

#[cfg(feature = "migrate")]
mod migrate;

/// An alias for [`Pool`][crate::pool::Pool], specialized for libSQL client.
pub type LibsqlClientPool = crate::pool::Pool<LibsqlClient>;

/// An alias for [`PoolOptions`][crate::pool::PoolOptions], specialized for libSQL client.
pub type LibsqlClientPoolOptions = crate::pool::PoolOptions<LibsqlClient>;

/// An alias for [`Executor<'_, Database = LibsqlClient>`][Executor].
pub trait LibsqlClientExecutor<'c>: Executor<'c, Database = LibsqlClient> {}
impl<'c, T: Executor<'c, Database = LibsqlClient>> LibsqlClientExecutor<'c> for T {}

// NOTE: required due to the lack of lazy normalization
impl_into_arguments_for_arguments!(LibsqlClientArguments);
impl_column_index_for_row!(LibsqlClientRow);
impl_column_index_for_statement!(LibsqlClientStatement);
impl_acquire!(LibsqlClient, LibsqlClientConnection);

// required because some databases have a different handling of NULL
impl_encode_for_option!(LibsqlClient);
