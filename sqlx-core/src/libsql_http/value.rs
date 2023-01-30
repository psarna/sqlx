use crate::database::{Database, HasValueRef};
use crate::decode::Decode;
use crate::error::{mismatched_types, Error};
use crate::libsql_http::{LibsqlHttp, LibsqlHttpTypeInfo};
use crate::type_info::TypeInfo;
use crate::types::Type;
use crate::value::{Value, ValueRef};
use libsql_client::CellValue;
use std::borrow::Cow;

enum LibsqlHttpValueData<'r> {
    Value(&'r LibsqlHttpValue),
}

pub struct LibsqlHttpValueRef<'r>(LibsqlHttpValueData<'r>);

impl<'r> LibsqlHttpValueRef<'r> {
    pub(crate) fn value(value: &'r LibsqlHttpValue) -> Self {
        Self(LibsqlHttpValueData::Value(value))
    }
}

impl<'r> ValueRef<'r> for LibsqlHttpValueRef<'r> {
    type Database = LibsqlHttp;

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
    pub(crate) type_info: LibsqlHttpTypeInfo,
}

impl Value for LibsqlHttpValue {
    type Database = LibsqlHttp;

    fn as_ref(&self) -> LibsqlHttpValueRef<'_> {
        LibsqlHttpValueRef::value(self)
    }

    fn type_info(&self) -> Cow<'_, LibsqlHttpTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        self.value.is_some()
    }
}
