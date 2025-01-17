// Copyright (c) The hasp Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Metadata generated by hasp.

macro_rules! json_impls {
    ($t:ty) => {
        #[cfg(feature = "rusqlite")]
        impl ::rusqlite::types::FromSql for $t {
            #[inline]
            fn column_result(
                value: ::rusqlite::types::ValueRef<'_>,
            ) -> ::rusqlite::types::FromSqlResult<Self> {
                value.as_str().and_then(|v| {
                    serde_json::from_str(v)
                        .map_err(|err| ::rusqlite::types::FromSqlError::Other(Box::new(err)))
                })
            }
        }

        #[cfg(feature = "rusqlite")]
        impl ::rusqlite::types::ToSql for $t {
            fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput<'_>> {
                match serde_json::to_string(self) {
                    Ok(s) => Ok(s.into()),
                    Err(err) => Err(::rusqlite::Error::ToSqlConversionFailure(Box::new(err))),
                }
            }
        }
    };
}

mod directory;
mod directory_hash;
mod directory_version;
mod install;

pub use directory::*;
pub use directory_hash::*;
pub use directory_version::*;
pub use install::*;
