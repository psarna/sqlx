#![allow(clippy::rc_buffer)]

use std::sync::Arc;

use sqlx_core::column::ColumnIndex;
use sqlx_core::error::Error;
use sqlx_core::ext::ustr::UStr;
use sqlx_core::row::Row;
use sqlx_core::HashMap;

use crate::{LibsqlClient, LibsqlClientColumn, LibsqlClientValue, LibsqlClientValueRef};

/// Implementation of [`Row`] for SQLite.
pub struct LibsqlClientRow {
    pub(crate) values: Box<[LibsqlClientValue]>,
    pub(crate) columns: Arc<Vec<LibsqlClientColumn>>,
    pub(crate) column_names: Arc<HashMap<UStr, usize>>,
}

// Accessing values from the statement object is
// safe across threads as long as we don't call [sqlite3_step]

// we block ourselves from doing that by only exposing
// a set interface on [StatementHandle]

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
