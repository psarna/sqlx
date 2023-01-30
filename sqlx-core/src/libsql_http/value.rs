use crate::database::{Database, HasValueRef};
use crate::decode::Decode;
use crate::error::{mismatched_types, Error};
use crate::type_info::TypeInfo;
use crate::types::Type;
use libsql_client::CellValue;
use std::borrow::Cow;

enum LibsqlHttpValueData<'r> {
    Value(&'r LibsqlHttpValue),
}

pub struct LibsqlHttpValueRef<'r>(LibsqlHttpValueData<'r>);

impl<'r> ValueRef<'r> for LibsqlHttpValueRef<'r> {
    type Database = Libsql;

    fn to_owned(&self) -> LibsqlHttpValue {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.clone(),
        }
    }

    fn type_info(&self) -> Cow<'_, LibsqlHttpTypeInfo> {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.type_info(),
        }
    }

    fn is_null(&self) -> bool {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.is_null(),
        }
    }
}

#[derive(Clone)]
pub struct LibsqlHttpValue {
    pub(crate) value: Option<CellValue>,
    pub(crate) type_info: SqliteTypeInfo,
}

impl Value for LibsqlHttpValue {
    type Database = Libsql;

    fn as_ref(&self) -> LibsqlHttpValueRef<'_> {
        LibsqlHttpValueRef::value(self)
    }

    fn type_info(&self) -> Cow<'_, SqliteTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        value.is_some()
    }
}
