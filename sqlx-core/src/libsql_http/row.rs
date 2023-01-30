#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use crate::HashMap;

use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::row::Row;
use crate::libsql_http::{Libsql, LibsqlColumn, LibsqlHttpValue, LibsqlHttpValueRef};

/// Implementation of [`Row`] for Libsql.
pub struct LibsqlRow {
    pub(crate) values: Box<[LibsqlHttpValue]>,
    pub(crate) columns: Arc<Vec<LibsqlColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl crate::row::private_row::Sealed for LibsqlRow {}

unsafe impl Send for LibsqlRow {}
unsafe impl Sync for LibsqlRow {}

impl Row for LibsqlRow {
    type Database = Libsql;

    fn columns(&self) -> &[LibsqlColumn] {
        &self.columns
    }

    fn try_get_raw<I>(&self, index: I) -> Result<LibsqlHttpValueRef<'_>, Error>
    where
        I: ColumnIndex<Self>,
    {
        let index = index.index(self)?;
        Ok(LibsqlHttpValueRef::value(&self.values[index]))
    }
}

impl ColumnIndex<LibsqlRow> for &'_ str {
    fn index(&self, row: &LibsqlRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}