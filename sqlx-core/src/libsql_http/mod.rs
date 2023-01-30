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
use database::LibsqlHttp;
use type_info::LibsqlHttpTypeInfo;
use column::LibsqlHttpColumn;
use value::{LibsqlHttpValue, LibsqlHttpValueRef};
use row::LibsqlHttpRow;
use transaction::LibsqlHttpTransactionManager;
use query_result::LibsqlHttpQueryResult;
use arguments::{LibsqlHttpArguments, LibsqlHttpArgumentValue};
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
