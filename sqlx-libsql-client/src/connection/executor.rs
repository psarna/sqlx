use crate::{
    LibsqlClient, LibsqlClientConnection, LibsqlClientQueryResult, LibsqlClientRow,
    LibsqlClientStatement, LibsqlClientTypeInfo,
};
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
// use futures_util::{TryFutureExt, TryStreamExt};
use sqlx_core::describe::Describe;
use sqlx_core::error::Error;
use sqlx_core::executor::{Execute, Executor};
use sqlx_core::Either;

impl<'c> Executor<'c> for &'c mut LibsqlClientConnection {
    type Database = LibsqlClient;

    fn fetch_many<'e, 'q: 'e, E: 'q>(
        self,
        mut _query: E,
    ) -> BoxStream<'e, Result<Either<LibsqlClientQueryResult, LibsqlClientRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        /*let sql = query.sql();
        let arguments = query.take_arguments();
        let persistent = query.persistent() && arguments.is_some();

        Box::pin(
            self.worker
                .execute(sql, arguments, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .try_flatten_stream(),
        )*/
        println!("fetch_many");
        todo!()
    }

    fn fetch_optional<'e, 'q: 'e, E: 'q>(
        self,
        mut _query: E,
    ) -> BoxFuture<'e, Result<Option<LibsqlClientRow>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        /*let sql = query.sql();
        let arguments = query.take_arguments();
        let persistent = query.persistent() && arguments.is_some();

        Box::pin(async move {
            let stream = self
                .worker
                .execute(sql, arguments, self.row_channel_size, persistent)
                .map_ok(flume::Receiver::into_stream)
                .try_flatten_stream();

            futures_util::pin_mut!(stream);

            while let Some(res) = stream.try_next().await? {
                if let Either::Right(row) = res {
                    return Ok(Some(row));
                }
            }

            Ok(None)
        })*/
        println!("fetch_optional");
        todo!()
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        _sql: &'q str,
        _parameters: &[LibsqlClientTypeInfo],
    ) -> BoxFuture<'e, Result<LibsqlClientStatement<'q>, Error>>
    where
        'c: 'e,
    {
        /*Box::pin(async move {
            let statement = self.worker.prepare(sql).await?;

            Ok(LibsqlClientStatement {
                sql: sql.into(),
                ..statement
            })
        })*/
        println!("prepare_with");
        todo!()
    }

    #[doc(hidden)]
    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> BoxFuture<'e, Result<Describe<LibsqlClient>, Error>>
    where
        'c: 'e,
    {
        //Box::pin(self.worker.describe(sql))
        println!("describing {sql}");
        Box::pin(futures_util::future::ok(Describe {
            columns: Vec::new(),
            parameters: Some(Either::Right(1)),
            nullable: Vec::new(),
        }))
    }
}
