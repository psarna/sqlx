use std::borrow::Cow;

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::type_info::DataType;
use crate::types::Type;
use crate::{LibsqlClient, LibsqlClientTypeInfo, LibsqlClientValueRef};

impl Type<LibsqlClient> for str {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Text)
    }
}

impl<'q> Encode<'q, LibsqlClient> for &'q str {
    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text{ value: self.to_string() });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for &'r str {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        value.text()
    }
}

impl Type<LibsqlClient> for String {
    fn type_info() -> LibsqlClientTypeInfo {
        <&str as Type<LibsqlClient>>::type_info()
    }
}

impl<'q> Encode<'q, LibsqlClient> for String {
    fn encode(self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text{ value: self });

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text{ value: self.clone() } );

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for String {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //value.text().map(ToOwned::to_owned)
        todo!()
    }
}

impl Type<LibsqlClient> for Cow<'_, str> {
    fn type_info() -> LibsqlClientTypeInfo {
        <&str as Type<LibsqlClient>>::type_info()
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        <&str as Type<LibsqlClient>>::compatible(ty)
    }
}

impl<'q> Encode<'q, LibsqlClient> for Cow<'q, str> {
    fn encode(self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text{ value: self.to_string() });

        IsNull::No
    }

    fn encode_by_ref(&self, args: &mut Vec<libsql_client::Value>) -> IsNull {
        args.push(libsql_client::Value::Text{ value: self.to_string() });

        IsNull::No
    }
}

impl<'r> Decode<'r, LibsqlClient> for Cow<'r, str> {
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        //value.text().map(Cow::Borrowed)
        todo!()
    }
}
