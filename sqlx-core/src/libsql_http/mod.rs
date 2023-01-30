#![allow(unsafe_code)]

mod arguments;
mod column;
mod connection;
mod database;
mod query_result;
mod row;
mod transaction;
mod type_info;
mod value;

use connection::{LibsqlHttpConnectOptions, LibsqlHttpConnection};
use database::Libsql;
use type_info::LibsqlTypeInfo;
use column::LibsqlColumn;
use value::{LibsqlHttpValue, LibsqlHttpValueRef};
use row::LibsqlRow;
use transaction::LibsqlHttpTransactionManager;
use query_result::LibsqlHttpQueryResult;
use arguments::{LibsqlArguments, LibsqlArgumentValue};
use crate::statement::Statement;

use std::borrow::Cow;
use std::sync::Arc;
use crate::HashMap;
use crate::ext::ustr::UStr;
use crate::column::ColumnIndex;
use either::Either;
use crate::error::Error;


#[derive(Debug, Clone)]
#[allow(clippy::rc_buffer)]
pub struct LibsqlStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) parameters: usize,
    pub(crate) columns: Arc<Vec<LibsqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl<'q> Statement<'q> for LibsqlStatement<'q> {
    type Database = Libsql;

    fn to_owned(&self) -> LibsqlStatement<'static> {
        LibsqlStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[LibsqlTypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    fn columns(&self) -> &[LibsqlColumn] {
        &self.columns
    }

    impl_statement_query!(LibsqlArguments<'_>);
}

impl ColumnIndex<LibsqlStatement<'_>> for &'_ str {
    fn index(&self, statement: &LibsqlStatement<'_>) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
