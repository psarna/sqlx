use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for f32 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Float)
    }
}

impl<'q> Encode<'q, LibsqlClient> for f32 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Float{ value: (*self).into()});

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for f32 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<f32, BoxDynError> {
        //Ok(value.double() as f32)
        todo!()
    }
}

impl Type<LibsqlClient> for f64 {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Float)
    }
}

impl<'q> Encode<'q, LibsqlClient> for f64 {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Float{ value: *self});

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for f64 {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<f64, BoxDynError> {
        //Ok(value.double())
        todo!()
    }
}
