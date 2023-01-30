use crate::database::{Database, HasArguments, HasStatement, HasStatementCache, HasValueRef};
use crate::libsql_http::{LibsqlHttpConnection, LibsqlHttpTransactionManager, LibsqlHttpRow,
    LibsqlHttpQueryResult, LibsqlHttpColumn, LibsqlHttpTypeInfo, LibsqlHttpValue, LibsqlHttpValueRef,
    LibsqlHttpArguments, LibsqlHttpArgumentValue, LibsqlHttpStatement
};

/// LibsqlHttp database driver.
#[derive(Debug)]
pub struct LibsqlHttp;

impl Database for LibsqlHttp {
    type Connection = LibsqlHttpConnection;

    type TransactionManager = LibsqlHttpTransactionManager;

    type Row = LibsqlHttpRow;

    type QueryResult = LibsqlHttpQueryResult;

    type Column = LibsqlHttpColumn;

    type TypeInfo = LibsqlHttpTypeInfo;

    type Value = LibsqlHttpValue;
}

impl<'r> HasValueRef<'r> for LibsqlHttp {
    type Database = LibsqlHttp;

    type ValueRef = LibsqlHttpValueRef<'r>;
}

impl<'q> HasArguments<'q> for LibsqlHttp {
    type Database = LibsqlHttp;

    type Arguments = LibsqlHttpArguments<'q>;

    type ArgumentBuffer = Vec<LibsqlHttpArgumentValue<'q>>;
}

impl<'q> HasStatement<'q> for LibsqlHttp {
    type Database = LibsqlHttp;

    type Statement = LibsqlHttpStatement<'q>;
}

impl HasStatementCache for LibsqlHttp {}
