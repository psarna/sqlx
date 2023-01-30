use crate::column::Column;
use crate::ext::ustr::UStr;
use crate::sqlite::{LibsqlHttp, LibsqlHttpTypeInfo};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct LibsqlHttpColumn {
    pub(crate) name: UStr,
    pub(crate) ordinal: usize,
    pub(crate) type_info: LibsqlHttpTypeInfo,
}

impl crate::column::private_column::Sealed for LibsqlHttpColumn {}

impl Column for LibsqlHttpColumn {
    type Database = LibsqlHttp;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &*self.name
    }

    fn type_info(&self) -> &LibsqlHttpTypeInfo {
        &self.type_info
    }
}
