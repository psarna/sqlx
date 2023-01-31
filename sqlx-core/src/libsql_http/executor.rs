use crate::describe::Describe;
use crate::error::Error;
use crate::executor::{Execute, Executor};
use crate::libsql_http::{
    LibsqlHttp, LibsqlHttpConnection, LibsqlHttpQueryResult, LibsqlHttpRow, LibsqlHttpStatement,
    LibsqlHttpTypeInfo,
};
use ahash::AHashMap as HashMap;
use either::Either;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use futures_util::{TryFutureExt, TryStreamExt};
use std::sync::Arc;

impl<'c> Executor<'c> for &'c mut LibsqlHttpConnection {
    type Database = LibsqlHttp;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut query: E,
    ) -> BoxStream<'e, Result<Either<LibsqlHttpQueryResult, LibsqlHttpRow>, Error>>
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
    ) -> BoxFuture<'e, Result<Option<LibsqlHttpRow>, Error>>
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
        _parameters: &[LibsqlHttpTypeInfo],
    ) -> BoxFuture<'e, Result<LibsqlHttpStatement<'q>, Error>>
    where
        'c: 'e,
    {
        Box::pin(async move {
            Ok(LibsqlHttpStatement {
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
    ) -> BoxFuture<'e, Result<Describe<LibsqlHttp>, Error>>
    where
        'c: 'e,
    {
        unimplemented!()
    }
}
