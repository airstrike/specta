//! This module contains functions that are public for the sole reason of the macros.
//!
//! They will not be documented and may go through breaking changes without a major version bump!
//!
//! DO NOT USE THEM! You have been warned!

#[cfg(feature = "export")]
pub use ctor;

#[cfg(feature = "functions")]
pub use specta_macros::fn_datatype;

/// Functions used to construct `crate::datatype` types (they have private fields so can't be constructed directly).
/// We intentionally keep their fields private so we can modify them without a major version bump.
pub mod construct {
    use std::borrow::Cow;

    use crate::datatype::*;

    pub const fn r#struct(
        generics: Vec<GenericType>,
        fields: Vec<StructField>,
        tag: Option<Cow<'static, str>>,
    ) -> StructType {
        StructType {
            generics,
            fields,
            tag,
        }
    }
}
