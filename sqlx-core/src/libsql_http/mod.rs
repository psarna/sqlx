#![allow(unsafe_code)]

mod arguments;
mod column;
mod connection;
mod database;
mod executor;
mod query_result;
mod row;
mod transaction;
mod type_info;
mod value;

use crate::statement::Statement;
use arguments::{LibsqlHttpArgumentValue, LibsqlHttpArguments};
use column::LibsqlHttpColumn;
pub use connection::{LibsqlHttpConnectOptions, LibsqlHttpConnection};
pub use database::LibsqlHttp;
use query_result::LibsqlHttpQueryResult;
use row::LibsqlHttpRow;
use transaction::LibsqlHttpTransactionManager;
use type_info::LibsqlHttpTypeInfo;
use value::{LibsqlHttpValue, LibsqlHttpValueRef};

use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::HashMap;
use either::Either;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[allow(clippy::rc_buffer)]
pub struct LibsqlHttpStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) parameters: usize,
    pub(crate) columns: Arc<Vec<LibsqlHttpColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl<'q> Statement<'q> for LibsqlHttpStatement<'q> {
    type Database = LibsqlHttp;

    fn to_owned(&self) -> LibsqlHttpStatement<'static> {
        LibsqlHttpStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[LibsqlHttpTypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    fn columns(&self) -> &[LibsqlHttpColumn] {
        &self.columns
    }

    impl_statement_query!(LibsqlHttpArguments<'_>);
}

impl ColumnIndex<LibsqlHttpStatement<'_>> for &'_ str {
    fn index(&self, statement: &LibsqlHttpStatement<'_>) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
