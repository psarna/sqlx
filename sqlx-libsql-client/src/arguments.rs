use crate::encode::{Encode, IsNull};
use crate::error::Error;
use crate::LibsqlClient;
use atoi::atoi;
use libsqlite3_sys::SQLITE_OK;
use std::borrow::Cow;

pub(crate) use sqlx_core::arguments::*;

#[derive(Debug, Clone)]
/*pub enum LibsqlClientArgumentValue<'q> {
    Null,
    Text(Cow<'q, str>),
    Blob(Cow<'q, [u8]>),
    Double(f64),
    Int(i32),
    Int64(i64),
}*/

#[derive(Default, Debug, Clone)]
pub struct LibsqlClientArguments {
    pub(crate) values: Vec<libsql_client::Value>,
}

impl LibsqlClientArguments {
    pub(crate) fn add<T>(&mut self, value: T)
    where
        T: Encode<'static, LibsqlClient>,
    {
        if let IsNull::Yes = value.encode(&mut self.values) {
            self.values.push(libsql_client::Value::Null);
        }
    }
}

impl<'q> Arguments<'_> for LibsqlClientArguments {
    type Database = LibsqlClient;

    fn reserve(&mut self, len: usize, _size_hint: usize) {
        self.values.reserve(len);
    }

    fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, Self::Database>,
    {
        self.add(value)
    }
}
