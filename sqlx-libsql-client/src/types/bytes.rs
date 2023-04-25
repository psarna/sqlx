use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for [u8] {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Blob)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Blob | DataType::Text)
    }
}

impl<'q> Encode<'q, LibsqlClient> for &'q [u8] {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Blob {
            value: self.to_vec(),
        });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for &'r [u8] {
    fn decode(_value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.blob())
        todo!()
    }
}

impl Type<LibsqlClient> for Vec<u8> {
    fn type_info() -> LibsqlClientTypeInfo {
        <&[u8] as Type<LibsqlClient>>::type_info()
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        <&[u8] as Type<LibsqlClient>>::compatible(ty)
    }
}

impl<'q> Encode<'q, LibsqlClient> for Vec<u8> {
    fn encode(self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Blob { value: self });

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Blob {
            value: self.clone(),
        });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for Vec<u8> {
    fn decode(_value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.blob().to_owned())
        todo!()
    }
}
