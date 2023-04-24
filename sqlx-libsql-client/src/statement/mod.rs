use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::{LibsqlClient, LibsqlClientArguments, LibsqlClientColumn, LibsqlClientTypeInfo};
use sqlx_core::{Either, HashMap};
use std::borrow::Cow;
use std::sync::Arc;

pub(crate) use sqlx_core::statement::*;

#[derive(Debug, Clone)]
#[allow(clippy::rc_buffer)]
pub struct LibsqlClientStatement<'q> {
    pub(crate) sql: Cow<'q, str>,
    pub(crate) parameters: usize,
    pub(crate) columns: Arc<Vec<LibsqlClientColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl<'q> Statement<'q> for LibsqlClientStatement<'q> {
    type Database = LibsqlClient;

    fn to_owned(&self) -> LibsqlClientStatement<'static> {
        LibsqlClientStatement::<'static> {
            sql: Cow::Owned(self.sql.clone().into_owned()),
            parameters: self.parameters,
            columns: Arc::clone(&self.columns),
            column_names: Arc::clone(&self.column_names),
        }
    }

    fn sql(&self) -> &str {
        &self.sql
    }

    fn parameters(&self) -> Option<Either<&[LibsqlClientTypeInfo], usize>> {
        Some(Either::Right(self.parameters))
    }

    fn columns(&self) -> &[LibsqlClientColumn] {
        &self.columns
    }

    impl_statement_query!(LibsqlClientArguments);
}

impl ColumnIndex<LibsqlClientStatement<'_>> for &'_ str {
    fn index(&self, statement: &LibsqlClientStatement<'_>) -> Result<usize, Error> {
        statement
            .column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
