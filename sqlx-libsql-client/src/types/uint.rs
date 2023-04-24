use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for u8 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for u8 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer(*self as i64));

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for u8 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int().try_into()?)
    }
}

impl Type<LibsqlClient> for u16 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for u16 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self as i64 });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for u16 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(value.int().try_into()?)
    }
}

impl Type<LibsqlClient> for u32 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int64)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for u32 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self as i64 });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for u32 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.int64().try_into()?)
        todo!()
    }
}
