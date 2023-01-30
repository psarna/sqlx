use std::ptr::NonNull;
use std::slice::from_raw_parts;
use std::str::from_utf8;
use std::sync::Arc;

use crate::error::BoxDynError;
use crate::sqlite::type_info::DataType;
use crate::sqlite::{LibsqlHttp, LibsqlHttpTypeInfo};
use crate::value::{Value, ValueRef};
use std::borrow::Cow;

use libsql_client::CellValue;

enum LibsqlHttpValueData<'r> {
    Value(&'r LibsqlHttpValue),
}

pub struct LibsqlHttpValueRef<'r>(LibsqlHttpValueData<'r>);

impl<'r> LibsqlHttpValueRef<'r> {
    pub(crate) fn value(value: &'r LibsqlHttpValue) -> Self {
        Self(LibsqlHttpValueData::Value(value))
    }

    pub(super) fn int(&self) -> i32 {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.int(),
        }
    }

    pub(super) fn int64(&self) -> i64 {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.int64(),
        }
    }

    pub(super) fn double(&self) -> f64 {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.double(),
        }
    }

    pub(super) fn blob(&self) -> &'r [u8] {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.blob(),
        }
    }

    pub(super) fn text(&self) -> Result<&'r str, BoxDynError> {
        match self.0 {
            LibsqlHttpValueData::Value(v) => v.text(),
        }
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

impl LibsqlHttpValue {
    pub(crate) unsafe fn new(value: CellValue, type_info: LibsqlHttpTypeInfo) -> Self {
        Self {
            type_info,
            value,
        }
    }

    fn type_info_opt(&self) -> Option<LibsqlHttpTypeInfo> {
        self.value.map(|v| {
            match v {
                CellValue::Text(_) => LibsqlHttpTypeInfo::DataType::Text,
                CellValue::Float(_) => LibsqlHttpTypeInfo::DataType::Float,
                CellValue::Number(_) => LibsqlHttpTypeInfo::DataType::Number,
                CellValue::Bool(_) => LibsqlHttpTypeInfo::DataType::Bool,
            }
        });
    }
}

impl Value for LibsqlHttpValue {
    type Database = LibsqlHttp;

    fn as_ref(&self) -> LibsqlHttpValueRef<'_> {
        LibsqlHttpValueRef::value(self)
    }

    fn type_info(&self) -> Cow<'_, LibsqlHttpTypeInfo> {
        self.type_info_opt()
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.type_info))
    }

    fn is_null(&self) -> bool {
        self.value.is_none()
    }
}
