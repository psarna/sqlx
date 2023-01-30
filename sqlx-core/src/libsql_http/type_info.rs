use std::fmt::{self, Display, Formatter};
use std::os::raw::c_int;
use std::str::FromStr;

use crate::error::BoxDynError;
use crate::type_info::TypeInfo;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum DataType {
    Null,
    Number,
    Float,
    Text,
    Blob,
}

/// Type information for a SQLite type.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "offline", derive(serde::Serialize, serde::Deserialize))]
pub struct LibsqlHttpTypeInfo(pub(crate) DataType);

impl Display for LibsqlHttpTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}

impl TypeInfo for LibsqlHttpTypeInfo {
    fn is_null(&self) -> bool {
        matches!(self.0, DataType::Null)
    }

    fn name(&self) -> &str {
        match self.0 {
            DataType::Null => "NULL",
            DataType::Text => "TEXT",
            DataType::Float => "REAL",
            DataType::Blob => "BLOB",
            DataType::Number => "INTEGER",
        }
    }
}

// note: this implementation is particularly important as this is how the macros determine
//       what Rust type maps to what *declared* SQL type
// <https://www.sqlite.org/datatype3.html#affname>
impl FromStr for DataType {
    type Err = BoxDynError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        Ok(match &*s {
            "int4" => DataType::Number,
            "int8" => DataType::Number,
            "boolean" | "bool" => DataType::Bool,

            _ if s.contains("int") => DataType::Number,

            _ if s.contains("char") || s.contains("clob") || s.contains("text") => DataType::Text,

            _ if s.contains("blob") => DataType::Blob,

            _ if s.contains("real") || s.contains("floa") || s.contains("doub") => DataType::Float,

            _ => {
                return Err(format!("unknown type: `{}`", s).into());
            }
        })
    }
}

#[test]
fn test_data_type_from_str() -> Result<(), BoxDynError> {
    assert_eq!(DataType::Number, "INT4".parse()?);

    assert_eq!(DataType::Number, "INT".parse()?);
    assert_eq!(DataType::Number, "INTEGER".parse()?);
    assert_eq!(DataType::Number, "INTBIG".parse()?);
    assert_eq!(DataType::Number, "MEDIUMINT".parse()?);

    assert_eq!(DataType::Number, "BIGINT".parse()?);
    assert_eq!(DataType::Number, "UNSIGNED BIG INT".parse()?);
    assert_eq!(DataType::Number, "INT8".parse()?);

    assert_eq!(DataType::Text, "CHARACTER(20)".parse()?);
    assert_eq!(DataType::Text, "NCHAR(55)".parse()?);
    assert_eq!(DataType::Text, "TEXT".parse()?);
    assert_eq!(DataType::Text, "CLOB".parse()?);

    assert_eq!(DataType::Blob, "BLOB".parse()?);

    assert_eq!(DataType::Float, "REAL".parse()?);
    assert_eq!(DataType::Float, "FLOAT".parse()?);
    assert_eq!(DataType::Float, "DOUBLE PRECISION".parse()?);

    Ok(())
}
