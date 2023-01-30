use crate::database::{Database, HasArguments, HasStatement, HasStatementCache, HasValueRef};
use crate::libsql_http::{LibsqlHttpConnection, LibsqlHttpTransactionManager, LibsqlRow,
    LibsqlHttpQueryResult, LibsqlColumn, LibsqlTypeInfo, LibsqlHttpValue, LibsqlHttpValueRef,
    LibsqlArguments, LibsqlArgumentValue, LibsqlStatement
};

/// Libsql database driver.
#[derive(Debug)]
pub struct Libsql;

impl Database for Libsql {
    type Connection = LibsqlHttpConnection;

    type TransactionManager = LibsqlHttpTransactionManager;

    type Row = LibsqlRow;

    type QueryResult = LibsqlHttpQueryResult;

    type Column = LibsqlColumn;

    type TypeInfo = LibsqlTypeInfo;

    type Value = LibsqlHttpValue;
}

impl<'r> HasValueRef<'r> for Libsql {
    type Database = Libsql;

    type ValueRef = LibsqlHttpValueRef<'r>;
}

impl<'q> HasArguments<'q> for Libsql {
    type Database = Libsql;

    type Arguments = LibsqlArguments<'q>;

    type ArgumentBuffer = Vec<LibsqlArgumentValue<'q>>;
}

impl<'q> HasStatement<'q> for Libsql {
    type Database = Libsql;

    type Statement = LibsqlStatement<'q>;
}

impl HasStatementCache for Libsql {}
