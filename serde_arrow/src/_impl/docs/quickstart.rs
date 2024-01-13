//! # Quickstart guide
//!
//! **Contents**
//!
//! 1. [Working with date time objects](#working-with-date-time-objects)
//! 2. [Decimals](#decimals)
//! 3. [Dictionary encoding for strings](#dictionary-encoding-for-strings)
//! 4. [Working with enums](#working-with-enums)
//! 5. [Convert from arrow2 to arrow
//!    arrays](#convert-from-arrow2-to-arrow-arrays)
//!
//! ## Working with date time objects
//!
//! When using `chrono`'s `DateTime<Utc>` or  `NaiveDateTime`, the values are
//! per default encoded as strings. To stores them as  `Date64` columns, the
//! data type has to be modified.
//!
//! For example
//!
//! ```
//! # fn main() -> serde_arrow::_impl::PanicOnError<()> {
//! # use serde_arrow::_impl::arrow as arrow;
//! #
//! use arrow::datatypes::{DataType, Field};
//! use chrono::NaiveDateTime;
//!
//! use serde_arrow::{schema::{SchemaLike, TracingOptions}, utils::Items};
//!
//! let items: &[NaiveDateTime] = &[
//!     NaiveDateTime::from_timestamp_opt(12 * 60 * 60 * 24, 0).unwrap(),
//!     // ...
//! ];
//!
//! let fields = Vec::<Field>::from_samples(&Items(items), TracingOptions::default())?;
//! assert_eq!(fields[0].data_type(), &DataType::LargeUtf8);
//! # Ok(())
//! # }
//! ```
//!
//! The traced field `val` will be of type `Utf8`. To store it as `Date64`
//! field, modify the data type as in
//!
//! ```rust
//! # use serde_arrow::_impl::arrow as arrow;
//! #
//! use arrow::datatypes::{DataType, Field};
//! use serde_arrow::schema::Strategy;
//!
//! let field =  Field::new("item", DataType::Date64, false,)
//!     .with_metadata(Strategy::NaiveStrAsDate64.into());
//! ```
//!
//! Integer fields containing timestamps in milliseconds since the epoch can be
//! directly stored as `Date64`:
//!
//! ```ignore
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct Record {
//!     timestamp: i64,
//! }
//!
//! let records: &[Record] = &[
//!     Record { timestamp: 12 * 60 * 60 * 24 * 1000 },
//!     Record { timestamp: 9 * 60 * 60 * 24 * 1000 },
//! ];
//!
//! let mut fields = serialize_into_fields(records, Default::default()).unwrap();
//! find_field_mut(&mut fields, "timestmap").unwrap() = Field::new(
//!     "timestamp", DataType::Date64, false,
//! );
//! ```
//!
//! ## Decimals
//!
//! To serialize decimals, use the `Decimal128(precision, scale)` data type:
//!
//! ```rust
//! # fn main() -> serde_arrow::_impl::PanicOnError<()> {
//! # use serde_arrow::_impl::arrow as arrow;
//! use std::str::FromStr;
//!
//! use arrow::datatypes::Field;
//! use bigdecimal::BigDecimal;
//! use serde_json::json;
//!
//! use serde_arrow::{schema::SchemaLike, utils::Items};
//!
//! let items = [
//!     BigDecimal::from_str("1.23").unwrap(),
//!     BigDecimal::from_str("4.56").unwrap(),
//! ];
//!
//! let fields = Vec::<Field>::from_value(&json!([
//!     {"name": "item", "data_type": "Decimal128(5, 2)"},
//! ]))?;
//!
//! let arrays = serde_arrow::to_arrow(&fields, &Items(&items))?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Dictionary encoding for strings
//!
//! To encode strings with repeated values via a dictionary, the data type of
//! the corresponding field must be changed from `Utf8` or `LargeUtf8` to
//! `Dictionary`.
//!
//! For an existing field this can be done via:
//!
//! ```ignore
//! field.data_type = DataType::Dictionary(
//!     // the integer type used for the keys
//!     IntegerType::UInt32,
//!     // the data type of the values
//!     Box::new(DataType::Utf8),
//!     // serde_arrow does not support sorted dictionaries
//!     false,
//! );
//! ```
//!
//! To dictionary encode all string fields, set the `string_dictionary_encoding`
//! of `TracingOptions`, when tracing the fields:
//!
//! ```ignore
//! let fields = serialize_into_fields(
//!     &items,
//!     TracingOptions::default().string_dictionary_encoding(true),
//! )?;
//! ```
//!
//! ## Working with enums
//!
//! Rust enums correspond to arrow's union types and are supported by
//! `serde_arrow`. Both enums with fields and without are supported. Variants
//! without fields are mapped to null arrays. Only variants that are included in
//! the union field can be serialized or deserialized and the variants must have
//! the correct index. When using `serialize_to_fields` these requirements will
//! automatically be met.
//!
//! For example:
//!
//! ```ignore
//! enum MyEnum {
//!     VariantWithoutData,
//!     Pair(u32, u32),
//!     NewType(Inner),
//! }
//!
//! struct Inner {
//!     a: f32,
//!     b: f32,
//! }
//! ```
//!
//! will be mapped to the following arrow union:
//!
//! - `type = 0`: `Null`
//! - `type = 1`: `Struct { 0: u32, 1: u32 }`
//! - `type = 2`: `Struct { a: f32, b: f32 }`
//!
//! ## Convert from arrow2 to arrow arrays
//!
//! Both `arrow` and `arrow2` use the Arrow memory format. Thanks to this fact,
//! it is possible to convert arrays between both packages with minimal work
//! using their respective FFI interfaces:
//!
//! - [`arrow2::ffi::export_field_to_c`](https://docs.rs/arrow2/latest/arrow2/ffi/fn.export_field_to_c.html)
//! - [`arrow2::ffi_export_array_to_c`](https://docs.rs/arrow2/latest/arrow2/ffi/fn.export_array_to_c.html)
//! - [`arrow::ffi::ArrowArray::new`](https://docs.rs/arrow/latest/arrow/ffi/struct.ArrowArray.html#method.new)
//!
//! The arrow2 crate includes [a helper trait][arrow2-arrow2arrow] to perform
//! this conversion when used with the `arrow` feature.
//!
//! [arrow2-arrow2arrow]:
//!     https://docs.rs/arrow2/latest/arrow2/array/trait.Arrow2Arrow.html
