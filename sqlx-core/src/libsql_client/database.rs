use crate::database::{Database, HasArguments, HasStatement, HasStatementCache, HasValueRef};
use crate::libsql_client::{
    LibsqlClientArgumentValue, LibsqlClientArguments, LibsqlClientColumn, LibsqlClientConnection,
    LibsqlClientQueryResult, LibsqlClientRow, LibsqlClientStatement, LibsqlClientTransactionManager,
    LibsqlClientTypeInfo, LibsqlClientValue, LibsqlClientValueRef,
};

/// LibsqlClient database driver.
#[derive(Debug)]
pub struct LibsqlClient;

impl Database for LibsqlClient {
    type Connection = LibsqlClientConnection;

    type TransactionManager = LibsqlClientTransactionManager;

    type Row = LibsqlClientRow;

    type QueryResult = LibsqlClientQueryResult;

    type Column = LibsqlClientColumn;

    type TypeInfo = LibsqlClientTypeInfo;

    type Value = LibsqlClientValue;
}

impl<'r> HasValueRef<'r> for LibsqlClient {
    type Database = LibsqlClient;

    type ValueRef = LibsqlClientValueRef<'r>;
}

impl<'q> HasArguments<'q> for LibsqlClient {
    type Database = LibsqlClient;

    type Arguments = LibsqlClientArguments<'q>;

    type ArgumentBuffer = Vec<LibsqlClientArgumentValue<'q>>;
}

impl<'q> HasStatement<'q> for LibsqlClient {
    type Database = LibsqlClient;

    type Statement = LibsqlClientStatement<'q>;
}

impl HasStatementCache for LibsqlClient {}
