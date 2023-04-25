use crate::{
    Either, LibsqlClient, LibsqlClientArguments, LibsqlClientColumn, LibsqlClientConnectOptions,
    LibsqlClientConnection, LibsqlClientQueryResult, LibsqlClientRow,
    LibsqlClientTransactionManager, LibsqlClientTypeInfo,
};
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{StreamExt, TryFutureExt, TryStreamExt};

use sqlx_core::any::{
    Any, AnyArguments, AnyColumn, AnyConnectOptions, AnyConnectionBackend, AnyQueryResult, AnyRow,
    AnyStatement, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind,
};

use crate::type_info::DataType;
use sqlx_core::connection::{ConnectOptions, Connection};
use sqlx_core::database::Database;
use sqlx_core::describe::Describe;
use sqlx_core::executor::Executor;
use sqlx_core::transaction::TransactionManager;

sqlx_core::declare_driver_with_optional_migrate!(DRIVER = LibsqlClient);

impl AnyConnectionBackend for LibsqlClientConnection {
    fn name(&self) -> &str {
        <LibsqlClient as Database>::NAME
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, sqlx_core::Result<()>> {
        Connection::close(*self)
    }

    fn close_hard(self: Box<Self>) -> BoxFuture<'static, sqlx_core::Result<()>> {
        Connection::close_hard(*self)
    }

    fn ping(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        Connection::ping(self)
    }

    fn begin(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        LibsqlClientTransactionManager::begin(self)
    }

    fn commit(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        LibsqlClientTransactionManager::commit(self)
    }

    fn rollback(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        LibsqlClientTransactionManager::rollback(self)
    }

    fn start_rollback(&mut self) {
        LibsqlClientTransactionManager::start_rollback(self)
    }

    fn shrink_buffers(&mut self) {
        // NO-OP.
    }

    fn flush(&mut self) -> BoxFuture<'_, sqlx_core::Result<()>> {
        Connection::flush(self)
    }

    fn should_flush(&self) -> bool {
        Connection::should_flush(self)
    }

    #[cfg(feature = "migrate")]
    fn as_migrate(
        &mut self,
    ) -> sqlx_core::Result<&mut (dyn sqlx_core::migrate::Migrate + Send + 'static)> {
        Ok(self)
    }

    fn fetch_many<'q>(
        &'q mut self,
        query: &'q str,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxStream<'q, sqlx_core::Result<Either<AnyQueryResult, AnyRow>>> {
        let persistent = arguments.is_some();
        let args = arguments.map(map_arguments);

        /*Box::pin(
            self.worker
                .execute(query, args, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .try_flatten_stream()
                .map(
                    move |res: sqlx_core::Result<
                        Either<LibsqlClientQueryResult, LibsqlClientRow>,
                    >| match res? {
                        Either::Left(result) => Ok(Either::Left(map_result(result))),
                        Either::Right(row) => Ok(Either::Right(AnyRow::try_from(&row)?)),
                    },
                ),
        )*/
        todo!()
    }

    fn fetch_optional<'q>(
        &'q mut self,
        query: &'q str,
        arguments: Option<AnyArguments<'q>>,
    ) -> BoxFuture<'q, sqlx_core::Result<Option<AnyRow>>> {
        let persistent = arguments.is_some();
        let args = arguments.map(map_arguments);

        /*Box::pin(async move {
            let stream = self
                .worker
                .execute(query, args, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .await?;
            futures_util::pin_mut!(stream);

            if let Some(Either::Right(row)) = stream.try_next().await? {
                return Ok(Some(AnyRow::try_from(&row)?));
            }

            Ok(None)
        })*/
        todo!()
    }

    fn prepare_with<'c, 'q: 'c>(
        &'c mut self,
        sql: &'q str,
        _parameters: &[AnyTypeInfo],
    ) -> BoxFuture<'c, sqlx_core::Result<AnyStatement<'q>>> {
        Box::pin(async move {
            let statement = Executor::prepare_with(self, sql, &[]).await?;
            AnyStatement::try_from_statement(sql, &statement, statement.column_names.clone())
        })
    }

    fn describe<'q>(&'q mut self, sql: &'q str) -> BoxFuture<'q, sqlx_core::Result<Describe<Any>>> {
        Box::pin(async move { Executor::describe(self, sql).await?.try_into_any() })
    }
}

impl<'a> TryFrom<&'a LibsqlClientTypeInfo> for AnyTypeInfo {
    type Error = sqlx_core::Error;

    fn try_from(libsql_type: &'a LibsqlClientTypeInfo) -> Result<Self, Self::Error> {
        Ok(AnyTypeInfo {
            kind: match &libsql_type.0 {
                DataType::Null => AnyTypeInfoKind::Null,
                DataType::Int => AnyTypeInfoKind::Integer,
                DataType::Int64 => AnyTypeInfoKind::BigInt,
                DataType::Float => AnyTypeInfoKind::Double,
                DataType::Blob => AnyTypeInfoKind::Blob,
                DataType::Text => AnyTypeInfoKind::Text,
                _ => {
                    return Err(sqlx_core::Error::AnyDriverError(
                        format!(
                            "Any driver does not support the libSQL type {:?}",
                            libsql_type
                        )
                        .into(),
                    ))
                }
            },
        })
    }
}

impl<'a> TryFrom<&'a LibsqlClientColumn> for AnyColumn {
    type Error = sqlx_core::Error;

    fn try_from(col: &'a LibsqlClientColumn) -> Result<Self, Self::Error> {
        let type_info =
            AnyTypeInfo::try_from(&col.type_info).map_err(|e| sqlx_core::Error::ColumnDecode {
                index: col.name.to_string(),
                source: e.into(),
            })?;

        Ok(AnyColumn {
            ordinal: col.ordinal,
            name: col.name.clone().into(),
            type_info,
        })
    }
}

impl<'a> TryFrom<&'a LibsqlClientRow> for AnyRow {
    type Error = sqlx_core::Error;

    fn try_from(row: &'a LibsqlClientRow) -> Result<Self, Self::Error> {
        AnyRow::map_from(row, row.column_names.clone())
    }
}

impl<'a> TryFrom<&'a AnyConnectOptions> for LibsqlClientConnectOptions {
    type Error = sqlx_core::Error;

    fn try_from(opts: &'a AnyConnectOptions) -> Result<Self, Self::Error> {
        let mut opts_out = LibsqlClientConnectOptions::from_url(&opts.database_url)?;
        //opts_out.log_settings = opts.log_settings.clone();
        Ok(opts_out)
    }
}

/// Instead of `AnyArguments::convert_into()`, we can do a direct mapping and preserve the lifetime.
fn map_arguments(args: AnyArguments<'_>) -> LibsqlClientArguments {
    LibsqlClientArguments {
        values: args
            .values
            .0
            .into_iter()
            .map(|val| match val {
                AnyValueKind::Null => libsql_client::Value::Null,
                AnyValueKind::SmallInt(i) => libsql_client::Value::Integer { value: i as i64 },
                AnyValueKind::Integer(i) => libsql_client::Value::Integer { value: i as i64 },
                AnyValueKind::BigInt(i) => libsql_client::Value::Integer { value: i },
                AnyValueKind::Real(r) => libsql_client::Value::Float { value: r as f64 },
                AnyValueKind::Double(d) => libsql_client::Value::Float { value: d },
                AnyValueKind::Text(t) => libsql_client::Value::Text {
                    value: t.to_string(),
                },
                AnyValueKind::Blob(b) => libsql_client::Value::Blob { value: b.to_vec() },
                // AnyValueKind is `#[non_exhaustive]` but we should have covered everything
                _ => unreachable!("BUG: missing mapping for {:?}", val),
            })
            .collect(),
    }
}

fn map_result(res: LibsqlClientQueryResult) -> AnyQueryResult {
    AnyQueryResult {
        rows_affected: res.rows_affected(),
        last_insert_id: None,
    }
}
