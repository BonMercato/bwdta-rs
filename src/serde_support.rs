//! Serde serialization/deserialization support for DTA format.
//!
//! This module provides serialization and deserialization of Rust structs to/from DTA format strings
//! using the serde framework. Enable the `serde` feature to use this module.
//!
//! # Examples
//!
//! ```
//! use serde::{Serialize, Deserialize};
//! use bwdta::{DynamicRecordIdentifier, to_dta_string, from_dta_string};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Address {
//!     #[serde(rename = "STAMMKALK")]
//!     stammkalk: String,
//!     #[serde(rename = "$LANDKUNDA$")]  // $ prefix/suffix marks parameters
//!     landkunda: String,
//!     #[serde(rename = "aa")]
//!     customer_nr: String,
//! }
//!
//! let address = Address {
//!     stammkalk: "J".to_string(),
//!     landkunda: "J".to_string(),
//!     customer_nr: "809460".to_string(),
//! };
//!
//! let identifier = DynamicRecordIdentifier::new("ADR");
//! let output = to_dta_string(&address, identifier)?;
//! # Ok::<(), bwdta::DtaError>(())
//! ```

use serde::{Serialize, ser, de};

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

use super::*;

pub struct DtaSerializer<T: RecordIdentifier> {
    identifier: T,
    is_nested: bool,
}

impl<T: RecordIdentifier> DtaSerializer<T> {
    pub fn new(identifier: T) -> Self {
        Self {
            identifier,
            is_nested: false,
        }
    }
}

impl<T: RecordIdentifier> ser::Serializer for DtaSerializer<T> {
    type Ok = String;
    type Error = DtaError;

    type SerializeSeq = DtaSequenceSerializer<T>;
    type SerializeTuple = ser::Impossible<String, DtaError>;
    type SerializeTupleStruct = ser::Impossible<String, DtaError>;
    type SerializeTupleVariant = ser::Impossible<String, DtaError>;
    type SerializeMap = ser::Impossible<String, DtaError>;
    type SerializeStruct = DtaStructSerializer<T>;
    type SerializeStructVariant = ser::Impossible<String, DtaError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(if v { "J".to_string() } else { "N".to_string() })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string().replace(".", ","))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string().replace(".", ","))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(v.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support bytes".to_string(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }

    fn serialize_some<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(String::new())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(variant.to_string())
    }

    fn serialize_newtype_struct<V>(
        self,
        _: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<V>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        Err(DtaError::Serde(
            "DTA format doesn't support newtype variants".to_string(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // Sequences are allowed both at top level and nested
        Ok(DtaSequenceSerializer::new(self.identifier))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuples".to_string(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuple structs".to_string(),
        ))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuple variants".to_string(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support maps".to_string(),
        ))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.is_nested {
            return Err(DtaError::Serde(
                "Nested structs are not supported in DTA format".to_string(),
            ));
        }
        Ok(DtaStructSerializer::new(self.identifier))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support struct variants".to_string(),
        ))
    }
}

#[derive(Debug)]
pub enum DtaFieldResult {
    Value(String),
    Sequence(Vec<String>),
}

pub struct DtaFieldSerializer {
    field_name: &'static str,
}

impl DtaFieldSerializer {
    pub fn new(field_name: &'static str) -> Self {
        Self { field_name }
    }
}

impl ser::Serializer for DtaFieldSerializer {
    type Ok = DtaFieldResult;
    type Error = DtaError;

    type SerializeSeq = DtaFieldSequenceSerializer;
    type SerializeTuple = ser::Impossible<DtaFieldResult, DtaError>;
    type SerializeTupleStruct = ser::Impossible<DtaFieldResult, DtaError>;
    type SerializeTupleVariant = ser::Impossible<DtaFieldResult, DtaError>;
    type SerializeMap = ser::Impossible<DtaFieldResult, DtaError>;
    type SerializeStruct = ser::Impossible<DtaFieldResult, DtaError>;
    type SerializeStructVariant = ser::Impossible<DtaFieldResult, DtaError>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(if v {
            "J".to_string()
        } else {
            "N".to_string()
        }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string().replace(".", ",")))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string().replace(".", ",")))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(v.to_string()))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support bytes".to_string(),
        ))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(String::new()))
    }

    fn serialize_some<V>(self, value: &V) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(String::new()))
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(String::new()))
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Value(variant.to_string()))
    }

    fn serialize_newtype_struct<V>(
        self,
        _: &'static str,
        value: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<V>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &V,
    ) -> Result<Self::Ok, Self::Error>
    where
        V: ?Sized + Serialize,
    {
        Err(DtaError::Serde(
            "DTA format doesn't support newtype variants".to_string(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(DtaFieldSequenceSerializer::new(self.field_name))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuples".to_string(),
        ))
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuple structs".to_string(),
        ))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support tuple variants".to_string(),
        ))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support maps".to_string(),
        ))
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(DtaError::Serde(
            "Nested structs are not supported in DTA format".to_string(),
        ))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(DtaError::Serde(
            "DTA format doesn't support struct variants".to_string(),
        ))
    }
}

pub struct DtaFieldSequenceSerializer {
    field_name: &'static str,
    rows: Vec<String>,
}

impl DtaFieldSequenceSerializer {
    pub fn new(field_name: &'static str) -> Self {
        Self {
            field_name,
            rows: Vec::new(),
        }
    }
}

impl ser::SerializeSeq for DtaFieldSequenceSerializer {
    type Ok = DtaFieldResult;
    type Error = DtaError;

    fn serialize_element<E>(&mut self, value: &E) -> Result<(), Self::Error>
    where
        E: ?Sized + Serialize,
    {
        // Use the field name as the SKZ for sequence elements
        let field_identifier = DynamicRecordIdentifier::new(&self.field_name.to_uppercase());
        let serializer = DtaSerializer::new(field_identifier);
        let row_string = value.serialize(serializer)?;
        self.rows.push(row_string);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(DtaFieldResult::Sequence(self.rows))
    }
}

pub struct DtaSequenceSerializer<T: RecordIdentifier> {
    identifier: T,
    rows: Vec<String>,
}

impl<T: RecordIdentifier> DtaSequenceSerializer<T> {
    pub fn new(identifier: T) -> Self {
        Self {
            identifier,
            rows: Vec::new(),
        }
    }
}

impl<T: RecordIdentifier> ser::SerializeSeq for DtaSequenceSerializer<T> {
    type Ok = String;
    type Error = DtaError;

    fn serialize_element<E>(&mut self, value: &E) -> Result<(), Self::Error>
    where
        E: ?Sized + Serialize,
    {
        let serializer = DtaSerializer::<T>::new(self.identifier.clone());
        let row_string = value.serialize(serializer)?;
        self.rows.push(row_string);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.rows.join(""))
    }
}

pub struct DtaStructSerializer<T: RecordIdentifier> {
    dta_row: DtaRow<T>,
    sequence_rows: Vec<String>,
}

impl<T: RecordIdentifier> DtaStructSerializer<T> {
    pub fn new(identifier: T) -> Self {
        Self {
            dta_row: DtaRow::new(identifier),
            sequence_rows: Vec::new(),
        }
    }
}

impl<T: RecordIdentifier> ser::SerializeStruct for DtaStructSerializer<T> {
    type Ok = String;
    type Error = DtaError;

    fn serialize_field<V>(&mut self, key: &'static str, value: &V) -> Result<(), Self::Error>
    where
        V: ?Sized + Serialize,
    {
        // Create a special nested serializer that can detect sequences
        let nested_serializer = DtaFieldSerializer::new(key);
        match value.serialize(nested_serializer) {
            Ok(DtaFieldResult::Value(value_str)) => {
                if value_str.is_empty() {
                    return Ok(());
                }

                let all_params = T::required_params()
                    .iter()
                    .chain(T::optional_params().iter())
                    .cloned()
                    .collect::<Vec<_>>();

                if all_params.contains(&key.to_string())
                    || (key.starts_with("$") && key.ends_with("$"))
                    || key == "VART"
                {
                    self.dta_row.with_param(&key.replace("$", ""), &value_str)?;
                } else {
                    self.dta_row.with_data_field(key, &value_str);
                }
            }
            Ok(DtaFieldResult::Sequence(seq_rows)) => {
                self.sequence_rows.extend(seq_rows);
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let main_row = self.dta_row.to_dta_string()?;
        let all_rows = if self.sequence_rows.is_empty() {
            main_row
        } else {
            format!("{}{}", main_row, self.sequence_rows.join(""))
        };
        Ok(all_rows)
    }
}

/// Serializes a value to a DTA format string.
///
/// This is the main entry point for serde-based serialization. Fields can be marked
/// as parameters by surrounding the field name with `$` symbols in the `#[serde(rename)]`
/// attribute.
///
/// # Examples
///
/// ```
/// use serde::Serialize;
/// use bwdta::{DynamicRecordIdentifier, to_dta_string};
///
/// #[derive(Serialize)]
/// struct Record {
///     #[serde(rename = "$PARAM$")]  // This is a parameter
///     param_field: String,
///     #[serde(rename = "aa")]       // This is a data field
///     data_field: String,
/// }
///
/// let record = Record {
///     param_field: "value".to_string(),
///     data_field: "123".to_string(),
/// };
///
/// let identifier = DynamicRecordIdentifier::new("TEST");
/// let output = to_dta_string(&record, identifier)?;
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn to_dta_string<T: RecordIdentifier, S: Serialize>(
    value: &S,
    identifier: T,
) -> Result<String, DtaError> {
    let serializer = DtaSerializer::<T>::new(identifier);
    value.serialize(serializer)
}

/// A deserializer for the DTA format.
pub struct DtaDeserializer<T: RecordIdentifier> {
    identifier: T,
    row: DtaRow<T>,
}

impl<T: RecordIdentifier> DtaDeserializer<T> {
    pub fn new(identifier: T, input: &str) -> Result<Self, DtaError> {
        // Parse the DTA string first to extract the SKZ, then validate it matches
        let temp_row = parse_dta_string(input)?;
        
        // Validate that the identifier matches the parsed SKZ
        if temp_row.identifier().skz() != identifier.skz() {
            return Err(DtaError::ParseError(format!(
                "SKZ mismatch: expected {}, got {}",
                identifier.skz(),
                temp_row.identifier().skz()
            )));
        }
        
        // Create a new row with the correct identifier type and copy the data
        let mut row = DtaRow::new(identifier.clone());
        
        // Copy parameters
        for (key, value) in temp_row.params {
            row.params.insert(key, value);
        }
        
        // Copy data fields
        row.data_fields = temp_row.data_fields;
        
        Ok(Self { identifier, row })
    }
}

impl<'de, T: RecordIdentifier> de::Deserializer<'de> for DtaDeserializer<T> {
    type Error = DtaError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, DtaError>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(DtaMapAccess::new(self.row))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, DtaError>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, DtaError>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}

/// Map access for DTA format deserialization.
struct DtaMapAccess<T: RecordIdentifier> {
    row: DtaRow<T>,
    param_keys: Vec<String>,
    data_keys: Vec<String>,
    current_index: usize,
    is_params: bool,
}

impl<T: RecordIdentifier> DtaMapAccess<T> {
    fn new(row: DtaRow<T>) -> Self {
        let param_keys: Vec<String> = row.params.keys().cloned().collect();
        let data_keys: Vec<String> = row.data_fields.iter().map(|(k, _)| k.clone()).collect();
        Self {
            row,
            param_keys,
            data_keys,
            current_index: 0,
            is_params: true,
        }
    }
}

impl<'de, T: RecordIdentifier> de::MapAccess<'de> for DtaMapAccess<T> {
    type Error = DtaError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, DtaError>
    where
        K: de::DeserializeSeed<'de>,
    {
        // Try parameters first
        if self.is_params && self.current_index < self.param_keys.len() {
            let key = &self.param_keys[self.current_index];
            return seed.deserialize(de::value::StrDeserializer::new(key)).map(Some);
        }

        // Then try data fields
        if !self.is_params {
            let data_index = self.current_index - self.param_keys.len();
            if data_index < self.data_keys.len() {
                let key = &self.data_keys[data_index];
                return seed.deserialize(de::value::StrDeserializer::new(key)).map(Some);
            }
        }

        // Switch from params to data fields if needed
        if self.is_params && self.current_index >= self.param_keys.len() {
            self.is_params = false;
            return self.next_key_seed(seed);
        }

        Ok(None)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, DtaError>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = if self.is_params {
            &self.param_keys[self.current_index]
        } else {
            let data_index = self.current_index - self.param_keys.len();
            &self.data_keys[data_index]
        };

        self.current_index += 1;

        // Look for the value in parameters first, then data fields
        let value = self.row.params.get(key)
            .or_else(|| self.row.data_fields.iter().find(|(k, _)| k == key).map(|(_, v)| v))
            .ok_or_else(|| DtaError::Serde(format!("Missing value for key: {}", key)))?;

        seed.deserialize(de::value::StrDeserializer::new(value))
    }
}

/// Deserializes a DTA format string into a value.
///
/// This is the main entry point for serde-based deserialization. Fields can be marked
/// as parameters by surrounding the field name with `$` symbols in the `#[serde(rename)]`
/// attribute.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use bwdta::{DynamicRecordIdentifier, from_dta_string};
///
/// #[derive(Deserialize)]
/// struct Record {
///     #[serde(rename = "STAMMKALK")]  // This is a parameter
///     stammkalk: String,
///     #[serde(rename = "aa")]       // This is a data field
///     data_field: String,
/// }
///
/// let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþaaþ123 \n";
/// let record: Record = from_dta_string::<DynamicRecordIdentifier, _>(dta_string)?;
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn from_dta_string<'a, T: RecordIdentifier, D: de::Deserialize<'a>>(
    input: &'a str,
) -> Result<D, DtaError> {
    // First parse to get the SKZ from the DTA string
    let temp_row = parse_dta_string(input)?;
    let identifier = T::from_skz(&temp_row.identifier().skz());
    
    let deserializer = DtaDeserializer::new(identifier, input)?;
    D::deserialize(deserializer)
}

/// Deserializes a DTA format string into a value with a specific identifier.
///
/// Use this when you need to specify a particular identifier type that doesn't implement Default.
pub fn from_dta_string_with_identifier<'a, T: RecordIdentifier, D: de::Deserialize<'a>>(
    input: &'a str,
    identifier: T,
) -> Result<D, DtaError> {
    let deserializer = DtaDeserializer::new(identifier, input)?;
    D::deserialize(deserializer)
}
