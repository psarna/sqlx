use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};
use std::borrow::Cow;
use uuid::{fmt::Hyphenated, Uuid};

impl Type<LibsqlClient> for Uuid {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Blob)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Blob | DataType::Text)
    }
}

impl<'q> Encode<'q, LibsqlClient> for Uuid {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Blob(self.as_bytes().to_vec()));

        IsNull::No
    }
}

impl Decode<'_, LibsqlClient> for Uuid {
    fn decode(value: LibsqlClientValueRef<'_>) -> Result<Self, BoxDynError> {
        // construct a Uuid from the returned bytes
        Uuid::from_slice(value.blob()).map_err(Into::into)
    }
}

impl Type<LibsqlClient> for Hyphenated {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Text)
    }
}

impl<'q> Encode<'q, LibsqlClient> for Hyphenated {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text(self.to_string()));

        IsNull::No
    }
}

impl Decode<'_, LibsqlClient> for Hyphenated {
    fn decode(value: LibsqlClientValueRef<'_>) -> Result<Self, BoxDynError> {
        let uuid: Result<Uuid, BoxDynError> =
            Uuid::parse_str(&value.text().map(ToOwned::to_owned)?).map_err(Into::into);

        Ok(uuid?.hyphenated())
    }
}
