use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for i8 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for i8 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self as i64 });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for i8 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.int().try_into()?)
        todo!()
    }
}

impl Type<LibsqlClient> for i16 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for i16 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self as i64});

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for i16 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.int().try_into()?)
        todo!()
    }
}

impl Type<LibsqlClient> for i32 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for i32 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self as i64 });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for i32 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.int())
        todo!()
    }
}

impl Type<LibsqlClient> for i64 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Int64)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for i64 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: *self });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for i64 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //Ok(value.int64())
        todo!()
    }
}
