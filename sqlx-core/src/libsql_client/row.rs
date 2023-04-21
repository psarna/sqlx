#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use crate::HashMap;

use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::libsql_client::{
    LibsqlClient, LibsqlClientColumn, LibsqlClientValue, LibsqlClientValueRef,
};
use crate::row::Row;

/// Implementation of [`Row`] for LibsqlClient.
pub struct LibsqlClientRow {
    pub(crate) values: Box<[LibsqlClientValue]>,
    pub(crate) columns: Arc<Vec<LibsqlClientColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl crate::row::private_row::Sealed for LibsqlClientRow {}

unsafe impl Send for LibsqlClientRow {}
unsafe impl Sync for LibsqlClientRow {}

impl Row for LibsqlClientRow {
    type Database = LibsqlClient;

    fn columns(&self) -> &[LibsqlClientColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<LibsqlClientValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(LibsqlClientValueRef::value(&self.values[index]))
    }
}

impl ColumnIndex<LibsqlClientRow> for &'_ str {
    fn index(&self, row: &LibsqlClientRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
