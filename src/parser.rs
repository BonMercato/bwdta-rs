//! DTA file parsing functionality.
//!
//! This module provides functions for parsing DTA format strings and files
//! into [`DtaRow`] structures.

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
    string::ToString,
    vec::Vec,
};

use crate::error::DtaError;
use crate::identifiers::{DynamicRecordIdentifier, RecordIdentifier};
use crate::row::DtaRow;

/// Parses a DTA format string into a [`DtaRow`] with dynamic identifier.
///
/// The input format is: `þ<param1>þ<value1>þ<param2>þ<value2>þ...þ<data_field1>þ<data_value1>þ...\n`
/// 
/// Parameters (VART, SKZ, UEBER, etc.) can appear in any order but always come before data fields.
/// Only SKZ is required - VART defaults to "0" and UEBER defaults to "N" if not provided.
/// Data fields are identified by their short lowercase keys (like aa, ab, ac).
///
/// # Errors
///
/// Returns [`DtaError::ParseError`] if the input format is invalid or SKZ is missing.
///
/// # Examples
///
/// ```
/// use bwdta::parser::parse_dta_string;
///
/// let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
/// let row = parse_dta_string(dta_string)?;
///
/// assert_eq!(row.params.get("STAMMKALK"), Some(&"J".to_string()));
/// assert_eq!(row.params.get("LANDKUNDA"), Some(&"J".to_string()));
/// assert_eq!(row.data_fields.len(), 3);
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn parse_dta_string(input: &str) -> Result<DtaRow<DynamicRecordIdentifier>, DtaError> {
    let trimmed = input.trim_end_matches('\n');
    
    if !trimmed.starts_with('þ') {
        return Err(DtaError::ParseError("DTA string must start with 'þ'".to_string()));
    }

    let parts: Vec<&str> = trimmed.split('þ').collect();
    if parts.len() < 2 {
        return Err(DtaError::ParseError("Insufficient parts in DTA string".to_string()));
    }

    // Skip the empty first part (before the first þ)
    let mut parts_iter = parts.iter().skip(1);

    // Parse all key-value pairs until we find data fields
    let mut vart_value = None;
    let mut skz_value = None;
    let mut ueber_value = None;
    let mut params = HashMap::new();
    let mut data_fields = Vec::new();

    // Process all remaining parts as key-value pairs
    while let Some(key) = parts_iter.next() {
        let value = parts_iter.next().ok_or_else(|| {
            DtaError::ParseError(format!("Missing value for key: {}", key))
        })?;

        // Check if this looks like a data field (short lowercase key)
        if key.len() <= 3 && key.chars().all(|c| c.is_ascii_lowercase()) {
            // This is a data field, add all remaining as data fields
            data_fields.push((key.to_string(), value.trim_end().to_string()));
            
            // Add any remaining parts as data fields
            while let Some(key) = parts_iter.next() {
                if let Some(value) = parts_iter.next() {
                    data_fields.push((key.to_string(), value.trim_end().to_string()));
                }
            }
            break;
        }

        // This is a parameter
        match *key {
            "VART" => vart_value = Some(value.to_string()),
            "SKZ" => skz_value = Some(value.to_string()),
            "UEBER" => ueber_value = Some(value.to_string()),
            _ => {
                params.insert(key.to_string(), value.trim_end().to_string());
            }
        }
    }

    // Validate required fields
    let skz_value = skz_value.ok_or_else(|| {
        DtaError::ParseError("Missing SKZ parameter".to_string())
    })?;
    let identifier = DynamicRecordIdentifier::new(&skz_value);

    let vart_value = vart_value.unwrap_or_else(|| "0".to_string());
    let ueber_value = ueber_value.unwrap_or_else(|| "N".to_string());

    // Create the row with identifier
    let mut row = DtaRow::new(identifier);

    // Add all parameters
    row.params.insert("VART".to_string(), vart_value);
    row.params.insert("UEBER".to_string(), ueber_value);
    row.params.extend(params);
    row.data_fields = data_fields;

    Ok(row)
}

/// Parses a DTA format string into a [`DtaRow`] with a specific identifier type.
///
/// This function requires that the identifier type implements `Default` so it can
/// create an instance to validate the SKZ match.
///
/// # Errors
///
/// Returns [`DtaError::ParseError`] if the input format is invalid.
/// Returns [`DtaError::ParseError`] if the SKZ doesn't match the expected identifier type.
///
/// # Examples
///
/// ```
/// use bwdta::{parser::parse_dta_string_with_identifier, identifiers::AddressIdentifier};
///
/// let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
/// let row = parse_dta_string_with_identifier::<AddressIdentifier>(dta_string)?;
///
/// assert_eq!(row.params.get("STAMMKALK"), Some(&"J".to_string()));
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn parse_dta_string_with_identifier<T: RecordIdentifier + Default>(
    input: &str,
) -> Result<DtaRow<T>, DtaError> {
    let dynamic_row = parse_dta_string(input)?;
    
    // Create an instance of the target identifier to get its expected SKZ
    let identifier = T::default();
    let expected_skz = identifier.skz();
    let actual_skz = dynamic_row.identifier.skz();
    
    if expected_skz != actual_skz {
        return Err(DtaError::ParseError(format!(
            "SKZ mismatch: expected '{}', got '{}'", 
            expected_skz, 
            actual_skz
        )));
    }

    // Create a new row with the typed identifier
    let mut row = DtaRow::new(identifier);
    row.params = dynamic_row.params;
    row.data_fields = dynamic_row.data_fields;

    // Validate the row against the identifier's requirements
    row.validate()?;

    Ok(row)
}

/// Parses multiple DTA rows from a string containing multiple lines.
///
/// Each line should be a complete DTA record ending with a newline.
///
/// # Errors
///
/// Returns [`DtaError::ParseError`] if any line has an invalid format.
///
/// # Examples
///
/// ```
/// use bwdta::parser::parse_dta_rows;
///
/// let dta_content = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþaaþ809460 \n\
///                    þVARTþ0þSKZþPOSþUEBERþNþaaþ1 þabþART001 \n";
/// 
/// let rows = parse_dta_rows(dta_content)?;
/// assert_eq!(rows.len(), 2);
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn parse_dta_rows(input: &str) -> Result<Vec<DtaRow<DynamicRecordIdentifier>>, DtaError> {
    let mut rows = Vec::new();
    
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let row = parse_dta_string(trimmed)?;
        rows.push(row);
    }

    Ok(rows)
}

#[cfg(all(feature = "std", feature = "io"))]
/// Reads and parses a DTA file from the given path.
///
/// # Errors
///
/// Returns [`DtaError::Io`] if the file cannot be read.
/// Returns [`DtaError::ParseError`] if the file contains invalid DTA format.
///
/// # Examples
///
/// ```no_run
/// use bwdta::parser::parse_dta_file;
///
/// let rows = parse_dta_file("data.dta")?;
/// println!("Parsed {} rows", rows.len());
/// # Ok::<(), bwdta::DtaError>(())
/// ```
pub fn parse_dta_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<DtaRow<DynamicRecordIdentifier>>, DtaError> {
    let content = std::fs::read_to_string(path)?;
    parse_dta_rows(&content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::AddressIdentifier;

    #[test]
    fn test_parse_simple_dta_string() {
        let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
        let row = parse_dta_string(dta_string).unwrap();

        assert_eq!(row.params.get("VART"), Some(&"0".to_string()));
        assert_eq!(row.params.get("UEBER"), Some(&"N".to_string()));
        assert_eq!(row.params.get("STAMMKALK"), Some(&"J".to_string()));
        assert_eq!(row.params.get("LANDKUNDA"), Some(&"J".to_string()));
        assert_eq!(row.data_fields.len(), 3);
        
        let mut sorted_fields = row.data_fields.clone();
        sorted_fields.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(sorted_fields[0], ("aa".to_string(), "809460".to_string()));
        assert_eq!(sorted_fields[1], ("ac".to_string(), "Claas".to_string()));
        assert_eq!(sorted_fields[2], ("ad".to_string(), "Elke".to_string()));
    }

    #[test]
    fn test_parse_dta_string_with_identifier() {
        let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
        let row = parse_dta_string_with_identifier::<AddressIdentifier>(dta_string).unwrap();

        assert_eq!(row.params.get("STAMMKALK"), Some(&"J".to_string()));
        assert_eq!(row.params.get("LANDKUNDA"), Some(&"J".to_string()));
        assert_eq!(row.data_fields.len(), 3);
    }

    #[test]
    fn test_parse_multiple_rows() {
        let dta_content = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþaaþ809460 \n\
                           þVARTþ0þSKZþPOSþUEBERþNþaaþ1 þabþART001 \n\
                           þVARTþ0þSKZþPOSþUEBERþNþaaþ2 þabþART002 \n";
        
        let rows = parse_dta_rows(dta_content).unwrap();
        assert_eq!(rows.len(), 3);
        
        assert_eq!(rows[0].identifier.skz(), "ADR");
        assert_eq!(rows[1].identifier.skz(), "POS");
        assert_eq!(rows[2].identifier.skz(), "POS");
    }

    #[test]
    fn test_parse_invalid_string() {
        let invalid_string = "VARTþ0þSKZþADR"; // Missing leading þ
        assert!(parse_dta_string(invalid_string).is_err());
    }

    #[test]
    fn test_parse_minimal_dta_string() {
        let dta_string = "þVARTþ0þSKZþADRþUEBERþN\n";
        let row = parse_dta_string(dta_string).unwrap();

        assert_eq!(row.params.get("VART"), Some(&"0".to_string()));
        assert_eq!(row.params.get("UEBER"), Some(&"N".to_string()));
        assert_eq!(row.data_fields.len(), 0);
    }

    #[test]
    fn test_round_trip() {
        let original = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
        let parsed = parse_dta_string(original).unwrap();
        let serialized = parsed.to_dta_string().unwrap();
        
        // Both should be equivalent DTA strings
        assert_eq!(original, serialized);
    }

    #[test]
    fn test_arbitrary_parameter_order() {
        // Test with parameters in different order
        let dta_string = "þVARTþ1þSKZþADRþUEBERþJþLANDKUNDAþNþSTAMMKALKþJþaaþ12345 þabþTest \n";
        let row = parse_dta_string(dta_string).unwrap();

        assert_eq!(row.params.get("VART"), Some(&"1".to_string()));
        assert_eq!(row.params.get("UEBER"), Some(&"J".to_string()));
        assert_eq!(row.params.get("LANDKUNDA"), Some(&"N".to_string()));
        assert_eq!(row.params.get("STAMMKALK"), Some(&"J".to_string()));
        assert_eq!(row.data_fields.len(), 2);
        
        let mut sorted_fields = row.data_fields.clone();
        sorted_fields.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(sorted_fields[0], ("aa".to_string(), "12345".to_string()));
        assert_eq!(sorted_fields[1], ("ab".to_string(), "Test".to_string()));
    }

    #[test]
    fn test_vart_not_first_parameter() {
        // Test with VART in different positions
        let dta_string1 = "þSKZþADRþVARTþ1þUEBERþJþSTAMMKALKþJþaaþ12345 \n";
        let row1 = parse_dta_string(dta_string1).unwrap();
        assert_eq!(row1.params.get("VART"), Some(&"1".to_string()));
        assert_eq!(row1.identifier().skz(), "ADR");
        assert_eq!(row1.params.get("UEBER"), Some(&"J".to_string()));
        assert_eq!(row1.data_fields.len(), 1);

        let dta_string2 = "þUEBERþNþSKZþADRþSTAMMKALKþJþVARTþ0þaaþ54321 \n";
        let row2 = parse_dta_string(dta_string2).unwrap();
        assert_eq!(row2.params.get("VART"), Some(&"0".to_string()));
        assert_eq!(row2.identifier().skz(), "ADR");
        assert_eq!(row2.params.get("UEBER"), Some(&"N".to_string()));
        assert_eq!(row2.data_fields.len(), 1);

        let dta_string3 = "þSTAMMKALKþJþVARTþ2þSKZþADRþUEBERþYþaaþ11111 \n";
        let row3 = parse_dta_string(dta_string3).unwrap();
        assert_eq!(row3.params.get("VART"), Some(&"2".to_string()));
        assert_eq!(row3.identifier().skz(), "ADR");
        assert_eq!(row3.params.get("UEBER"), Some(&"Y".to_string()));
        assert_eq!(row3.data_fields.len(), 1);
    }
}
