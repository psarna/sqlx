#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use crate::HashMap;

use crate::column::ColumnIndex;
use crate::error::Error;
use crate::ext::ustr::UStr;
use crate::libsql_http::{LibsqlHttp, LibsqlHttpColumn, LibsqlHttpValue, LibsqlHttpValueRef};
use crate::row::Row;

/// Implementation of [`Row`] for LibsqlHttp.
pub struct LibsqlHttpRow {
    pub(crate) values: Box<[LibsqlHttpValue]>,
    pub(crate) columns: Arc<Vec<LibsqlHttpColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

impl crate::row::private_row::Sealed for LibsqlHttpRow {}

unsafe impl Send for LibsqlHttpRow {}
unsafe impl Sync for LibsqlHttpRow {}

impl Row for LibsqlHttpRow {
    type Database = LibsqlHttp;

    fn columns(&self) -> &[LibsqlHttpColumn] {
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

impl ColumnIndex<LibsqlHttpRow> for &'_ str {
    fn index(&self, row: &LibsqlHttpRow) -> Result<usize, Error> {
        row.column_names
            .get(*self)
            .ok_or_else(|| Error::ColumnNotFound((*self).into()))
            .map(|v| *v)
    }
}
