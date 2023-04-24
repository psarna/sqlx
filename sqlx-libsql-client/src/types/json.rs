use serde::{Deserialize, Serialize};

use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::{Json, Type};
use crate::{
    type_info::DataType, LibsqlClient, LibsqlClientArgumentValue, LibsqlClientTypeInfo,
    LibsqlClientValueRef,
};

impl<T> Type<LibsqlClient> for Json<T> {
    fn type_info() -> LibsqlClientTypeInfo {
        LibsqlClientTypeInfo(DataType::Text)
    }

    fn compatible(ty: &LibsqlClientTypeInfo) -> bool {
        <&str as Type<LibsqlClient>>::compatible(ty)
    }
}

impl<T> Encode<'_, LibsqlClient> for Json<T>
where
    T: Serialize,
{
    fn encode_by_ref(&self, buf: &mut Vec<LibsqlClientArgumentValue<'_>>) -> IsNull {
        Encode::<LibsqlClient>::encode(self.encode_to_string(), buf)
    }
}

impl<'r, T> Decode<'r, LibsqlClient> for Json<T>
where
    T: 'r + Deserialize<'r>,
{
    fn decode(value: LibsqlClientValueRef<'r>) -> Result<Self, BoxDynError> {
        Self::decode_from_string(Decode::<LibsqlClient>::decode(value)?)
    }
}
