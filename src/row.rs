//! DTA row structures and builders.
//!
//! This module provides [`DtaRow`] for representing a single DTA record and
//! [`DtaRowBuilder`] for constructing rows with a fluent API.

#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap as HashMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::{
    collections::HashMap,
    format,
    string::{String, ToString},
    vec::Vec,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::error::DtaError;
use crate::identifiers::RecordIdentifier;

/// Represents a single DTA record with parameters and data fields.
///
/// A DTA row consists of:
/// - A record identifier (SKZ) that defines the record type
/// - Parameters (e.g., STAMMKALK, LANDKUNDA)
/// - Data fields with short codes (e.g., aa, ab, ac)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DtaRow<T: RecordIdentifier> {
    /// Parameters for this record (e.g., STAMMKALK, LANDKUNDA).
    pub params: HashMap<String, String>,
    /// Data fields as key-value pairs (e.g., "aa" -> "809460").
    pub data_fields: Vec<(String, String)>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub(crate) identifier: T,
}

impl<T: RecordIdentifier> DtaRow<T> {
    /// Creates a new empty DTA row with the given identifier.
    #[must_use]
    pub fn new(identifier: T) -> Self {
        Self {
            params: HashMap::new(),
            data_fields: Vec::new(),
            identifier,
        }
    }

    /// Adds a parameter to this row.
    ///
    /// # Errors
    ///
    /// Returns [`DtaError::InvalidParam`] if the parameter is not valid for this record type
    /// (when parameter validation is enabled).
    pub fn with_param<S: Into<String>>(&mut self, key: S, value: S) -> Result<&mut Self, DtaError> {
        let key_str = key.into();
        let all_params = T::required_params()
            .iter()
            .chain(T::optional_params().iter())
            .cloned()
            .collect::<Vec<_>>();

        if T::check_params() && !all_params.contains(&key_str) {
            return Err(DtaError::InvalidParam(key_str));
        }

        self.params.insert(key_str, value.into());
        Ok(self)
    }

    /// Adds a data field to this row.
    ///
    /// Data fields are always accepted without validation.
    pub fn with_data_field<S: Into<String>>(&mut self, key: S, value: S) -> &mut Self {
        self.data_fields.push((key.into(), value.into()));
        self
    }

    /// Validates that all required parameters are present.
    ///
    /// # Errors
    ///
    /// Returns [`DtaError::MissingRequiredParameter`] if a required parameter is missing.
    pub fn validate(&self) -> Result<(), DtaError> {
        for required_param in T::required_params() {
            if T::check_params() && !self.params.contains_key(&required_param) {
                return Err(DtaError::MissingRequiredParameter(required_param));
            }
        }

        Ok(())
    }

    /// Converts this row to a DTA format string.
    ///
    /// The output format is: `þVARTþ0þSKZþ<skz>þUEBERþNþ<params>þ<data>\n`
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn to_dta_string(&self) -> Result<String, DtaError> {
        self.validate()?;

        let mut parts = Vec::new();
        parts.push("VART".to_string());

        if let Some(vart_value) = self.params.get("VART") {
            parts.push(vart_value.clone());
        } else {
            parts.push("0".to_string());
        }

        parts.push("SKZ".to_string());
        parts.push(self.identifier.skz());

        parts.push("UEBER".to_string());
        if let Some(ueber_value) = self.params.get("UEBER") {
            parts.push(ueber_value.clone());
        } else {
            parts.push("N".to_string());
        }

        let mut all_params = self.params.keys().cloned().collect::<Vec<_>>();

        for param in T::param_order() {
            if param != "VART" {
                if let Some(value) = self.params.get(&param) {
                    all_params.retain(|p| p != &param);
                    parts.push(param.to_string());
                    parts.push(value.clone());
                }
            }
        }

        // remaining unsorted params which don't have a fixed order
        for param in all_params {
            if let Some(value) = self.params.get(&param) {
                parts.push(param.to_string());
                parts.push(value.clone());
            }
        }

        let mut sorted_data_fields = self.data_fields.clone();
        sorted_data_fields.sort_by(|a, b| a.0.cmp(&b.0));

        for (key, value) in sorted_data_fields {
            parts.push(key);
            parts.push(format!("{} ", value));
        }

        Ok(format!("þ{}\n", parts.join("þ")))
    }
}

/// Builder for constructing [`DtaRow`] instances with a fluent API.
///
/// # Examples
///
/// ```
/// use bwdta::{DtaRowBuilder, DynamicRecordIdentifier};
///
/// let identifier = DynamicRecordIdentifier::new("ADR");
/// let row = DtaRowBuilder::new(identifier)
///     .param("STAMMKALK", "J")?
///     .data("aa", "809460")
///     .data("ac", "Claas")
///     .build()?;
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub struct DtaRowBuilder<T: RecordIdentifier> {
    row: DtaRow<T>,
}

impl<T: RecordIdentifier> DtaRowBuilder<T> {
    /// Creates a new builder with the given record identifier.
    #[must_use]
    pub fn new(identifier: T) -> Self {
        Self {
            row: DtaRow::new(identifier),
        }
    }

    /// Adds a parameter to the row being built.
    ///
    /// # Errors
    ///
    /// Returns [`DtaError::InvalidParam`] if the parameter is not valid for this record type.
    pub fn param<S: Into<String>>(mut self, key: S, value: S) -> Result<Self, DtaError> {
        self.row.with_param(key, value)?;
        Ok(self)
    }

    /// Adds a data field to the row being built.
    #[must_use]
    pub fn data<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.row.with_data_field(key, value);
        self
    }

    /// Builds the final [`DtaRow`], validating all required parameters are present.
    ///
    /// # Errors
    ///
    /// Returns [`DtaError::MissingRequiredParameter`] if a required parameter is missing.
    pub fn build(self) -> Result<DtaRow<T>, DtaError> {
        self.row.validate()?;
        Ok(self.row)
    }
}
