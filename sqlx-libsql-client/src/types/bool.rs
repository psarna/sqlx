use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for bool {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Bool)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        matches!(ty.0, DataType::Bool | DataType::Int | DataType::Int64)
    }
}

impl<'q> Encode<'q, LibsqlClient> for bool {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Integer{ value: (*self).into() });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for bool {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<bool, BoxDynError> {
        //Ok(value.int() != 0)
        todo!()
    }
}
