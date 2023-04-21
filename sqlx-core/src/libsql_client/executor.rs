use crate::describe::Describe;
use crate::error::Error;
use crate::executor::{Execute, Executor};
use crate::libsql_client::{
    LibsqlClient, LibsqlClientConnection, LibsqlClientQueryResult, LibsqlClientRow,
    LibsqlClientStatement, LibsqlClientTypeInfo,
};
use ahash::AHashMap as HashMap;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use std::sync::Arc;

impl<'c> Executor<'c> for &'c mut LibsqlClientConnection {
    type Database = LibsqlClient;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxStream<'e, Result<Either<LibsqlClientQueryResult, LibsqlClientRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        let sql = query.sql();
        let arguments = query.take_arguments();
        let persistent = query.persistent() && arguments.is_some();

        unimplemented!()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxFuture<'e, Result<Option<LibsqlClientRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        let sql = query.sql();
        let arguments = query.take_arguments();
        let persistent = query.persistent() && arguments.is_some();

        unimplemented!()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        _parameters: &[LibsqlClientTypeInfo],
    ) -> BoxFuture<'e, Result<LibsqlClientStatement<'q>, Error>>
    where
        'c: 'e,
    {
        Box::pin(async move {
            Ok(LibsqlClientStatement {
                sql: sql.into(),
                parameters: 0,                          // FIXME!!!!
                columns: Arc::new(vec![]),              // FIXME!!!!
                column_names: Arc::new(HashMap::new()), // FIXME!!!!
            })
        })
    }

    #[doc(hidden)]
    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<Describe<LibsqlClient>, Error>>
    where
        'c: 'e,
    {
        unimplemented!()
    }
}
