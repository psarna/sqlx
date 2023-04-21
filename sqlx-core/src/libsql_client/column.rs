use crate::column::Column;
use crate::ext::ustr::UStr;
use crate::libsql_client::{LibsqlClient, LibsqlClientTypeInfo};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct LibsqlClientColumn {
    pub(crate) name: UStr,
    pub(crate) ordinal: usize,
    pub(crate) type_info: LibsqlClientTypeInfo,
}

impl crate::column::private_column::Sealed for LibsqlClientColumn {}

impl Column for LibsqlClientColumn {
    type Database = LibsqlClient;

    fn ordinal(&self) -> usize {
        self.ordinal
    }

    fn name(&self) -> &str {
        &*self.name
    }

    fn type_info(&self) -> &LibsqlClientTypeInfo {
        &self.type_info
    }
}
