use crate::{LibsqlClient, LibsqlClientTypeInfo};

pub(crate) use sqlx_core::column::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct LibsqlClientColumn {
    pub(crate) name: String,
    pub(crate) ordinal: usize,
    pub(crate) type_info: LibsqlClientTypeInfo,
}

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
