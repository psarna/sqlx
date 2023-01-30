use crate::arguments::Arguments;
use crate::encode::{Encode, IsNull};
use crate::error::Error;
use atoi::atoi;
use std::borrow::Cow;
use crate::libsql_http::Libsql;

const LIBSQL_OK: i32 = 0;

#[derive(Debug, Clone)]
pub enum LibsqlArgumentValue<'q> {
    Null,
    Text(Cow<'q, str>),
    Blob(Cow<'q, [u8]>),
    Double(f64),
    Int(i32),
    Int64(i64),
}

#[derive(Default, Debug, Clone)]
pub struct LibsqlArguments<'q> {
    pub(crate) values: Vec<LibsqlArgumentValue<'q>>,
}

impl<'q> LibsqlArguments<'q> {
    pub(crate) fn add<T>(&mut self, value: T)
    where
        T: Encode<'q, Libsql>,
    {
        if let IsNull::Yes = value.encode(&mut self.values) {
            self.values.push(LibsqlArgumentValue::Null);
        }
    }
}

impl<'q> Arguments<'q> for LibsqlArguments<'q> {
    type Database = Libsql;

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