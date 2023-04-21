use crate::arguments::Arguments;
use crate::encode::{Encode, IsNull};
use crate::error::Error;
use crate::libsql_client::LibsqlClient;
use atoi::atoi;
use std::borrow::Cow;

const LIBSQL_OK: i32 = 0;

#[derive(Debug, Clone)]
pub enum LibsqlClientArgumentValue<'q> {
    Null,
    Text(Cow<'q, str>),
    Blob(Cow<'q, [u8]>),
    Double(f64),
    Int(i32),
    Int64(i64),
}

#[derive(Default, Debug, Clone)]
pub struct LibsqlClientArguments<'q> {
    pub(crate) values: Vec<LibsqlClientArgumentValue<'q>>,
}

impl<'q> LibsqlClientArguments<'q> {
    pub(crate) fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, LibsqlClient>,
    {
        if let IsNull::Yes = value.encode(&mut self.values) {
            self.values.push(LibsqlClientArgumentValue::Null);
        }
    }
}

impl<'q> Arguments<'q> for LibsqlClientArguments<'q> {
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
