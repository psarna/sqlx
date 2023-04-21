use crate::libsql_client::{LibsqlClient, LibsqlClientTypeInfo};
use crate::value::{Value, ValueRef};
use std::borrow::Cow;

enum LibsqlClientValueData<'r> {
    Value(&'r LibsqlClientValue),
}

pub struct LibsqlClientValueRef<'r>(LibsqlClientValueData<'r>);

impl<'r> LibsqlClientValueRef<'r> {
    pub(crate) fn value(value: &'r LibsqlClientValue) -> Self {
        Self(LibsqlClientValueData::Value(value))
    }
}

impl<'r> ValueRef<'r> for LibsqlClientValueRef<'r> {
    type Database = LibsqlClient;

    fn to_owned(&self) -> LibsqlClientValue {
        match self.0 {
            LibsqlClientValueData::Value(v) => v.clone(),
        }
    }

    fn type_info(&self) -> Cow<'_, LibsqlClientTypeInfo> {
        match self.0 {
            LibsqlClientValueData::Value(v) => v.type_info(),
        }
    }

    fn is_null(&self) -> bool {
        match self.0 {
            LibsqlClientValueData::Value(v) => v.is_null(),
        }
    }
}

#[derive(Clone)]
pub struct LibsqlClientValue {
    pub(crate) value: Option<libsql_client::Value>,
    pub(crate) type_info: LibsqlClientTypeInfo,
}

impl Value for LibsqlClientValue {
    type Database = LibsqlClient;

    fn as_ref(&self) -> LibsqlClientValueRef<'_> {
        LibsqlClientValueRef::value(self)
    }

    fn type_info(&self) -> Cow<'_, LibsqlClientTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    fn is_null(&self) -> bool {
        self.value.is_some()
    }
}
