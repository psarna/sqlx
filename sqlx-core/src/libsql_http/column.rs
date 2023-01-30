use crate::column::Column;
use crate::ext::ustr::UStr;
use crate::libsql_http::{Libsql, LibsqlTypeInfo};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct LibsqlColumn {
    pub(crate) name: UStr,
    pub(crate) ordinal: usize,
    pub(crate) type_info: LibsqlTypeInfo,
}

impl crate::column::private_column::Sealed for LibsqlColumn {}

impl Column for LibsqlColumn {
    type Database = Libsql;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &*self.name
    }

    fn type_info(&self) -> &LibsqlTypeInfo {
        &self.type_info
    }
}
