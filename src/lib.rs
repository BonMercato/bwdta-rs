//! A library for working with the DTA (Data Transfer) format used by ERPSuite
//! (formerly BüroWARE), a German ERP software by SoftENGINE.
//!
//! This library provides tools for constructing and serializing DTA records with
//! type-safe record identifiers, parameter validation, and optional serde support.
//!
//! # Features
//!
//! - **Builder pattern** for constructing DTA rows
//! - **Serde support** for serializing Rust structs (requires `serde` feature)
//! - **no_std compatible** (disable default features)
//! - **Predefined identifiers** for common record types
//! - **Parameter validation** to ensure data integrity
//!
//! # Examples
//!
//! ```
//! use bwdta::{DtaRowBuilder, DynamicRecordIdentifier};
//!
//! let identifier = DynamicRecordIdentifier::new("ADR");
//! let row = DtaRowBuilder::new(identifier)
//!     .param("STAMMKALK", "J")?
//!     .data("aa", "809460")
//!     .data("ac", "Claas")
//!     .build()?;
//!
//! let output = row.to_dta_string()?;
//! # Ok::<(), bwdta::DtaError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod error;
pub mod identifiers;
pub mod parser;
pub mod row;
#[cfg(all(feature = "std", feature = "io"))]
pub mod writer;

#[cfg(feature = "serde")]
mod serde_support;

pub use error::DtaError;
pub use identifiers::{
    DEFAULT_OPTIONAL_PARAMS, DEFAULT_PARAM_ORDER, DEFAULT_REQUIRED_PARAMS, DynamicRecordIdentifier,
    RecordIdentifier,
};
pub use parser::{parse_dta_file, parse_dta_string, parse_dta_string_with_identifier, parse_dta_rows};
pub use row::{DtaRow, DtaRowBuilder};
#[cfg(feature = "serde")]
pub use serde_support::*;
#[cfg(all(feature = "std", feature = "io"))]
pub use writer::DtaWriter;

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "serde", not(feature = "std")))]
    use alloc::{
        string::{String, ToString},
        vec::Vec,
    };

    #[cfg(feature = "serde")]
    use serde::Serialize;

    #[cfg(all(feature = "serde", not(feature = "std")))]
    use alloc::vec;

    use super::*;

    #[test]
    fn test_basic_row_creation() {
        let identifier = DynamicRecordIdentifier::new("ADR");
        let row = DtaRowBuilder::new(identifier)
            .param("STAMMKALK", "J")
            .unwrap()
            .param("LANDKUNDA", "J")
            .unwrap()
            .data("aa", "809460")
            .data("ac", "Claas")
            .data("ad", "Elke")
            .build()
            .unwrap();

        let output = row.to_dta_string().unwrap();
        assert_eq!(
            output,
            "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n"
        );
    }

    #[test]
    #[should_panic]
    fn test_basic_row_creation_unknown_param() {
        let identifier = identifiers::AddressIdentifier;
        DtaRowBuilder::new(identifier)
            .param("LANDKUNDA", "J")
            .unwrap()
            .param("MYPARAM", "J")
            .unwrap()
            .data("aa", "809460")
            .data("ac", "Claas")
            .data("ad", "Elke")
            .build()
            .unwrap();
    }

    #[cfg(feature = "serde")]
    #[derive(Serialize)]
    struct TestRow {
        #[serde(rename = "STAMMKALK")]
        stammkalk: String,
        #[serde(rename = "$LANDKUNDA$")]
        landkunda: String,
        #[serde(rename = "aa")]
        customer_nr: String,
        #[serde(rename = "ac")]
        first_name: String,
        #[serde(rename = "ad")]
        last_name: String,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        let identifier = DynamicRecordIdentifier::new("ADR");
        let row = TestRow {
            stammkalk: "J".to_string(),
            landkunda: "J".to_string(),
            customer_nr: "809460".to_string(),
            first_name: "Claas".to_string(),
            last_name: "Elke".to_string(),
        };

        let serialized = serde_support::to_dta_string(&row, identifier).unwrap();
        assert_eq!(
            serialized,
            "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n"
        );
    }

    #[cfg(feature = "serde")]
    #[derive(Serialize)]
    struct BelRow {
        #[serde(rename = "STAMMKALK")]
        stammkalk: String,
        #[serde(rename = "aa")]
        customer_nr: String,
        #[serde(rename = "ac")]
        first_name: String,

        #[serde(rename = "POS")]
        positions: Vec<PosRow>,
    }

    #[cfg(feature = "serde")]
    #[derive(Serialize)]
    struct PosRow {
        #[serde(rename = "aa")]
        position_nr: String,
        #[serde(rename = "ab")]
        article_code: String,
        #[serde(rename = "ac")]
        quantity: String,
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_sequence_serialization() {
        let identifier = DynamicRecordIdentifier::new("BEL");
        let row = BelRow {
            stammkalk: "J".to_string(),
            customer_nr: "809460".to_string(),
            first_name: "Claas".to_string(),
            positions: vec![
                PosRow {
                    position_nr: "1".to_string(),
                    article_code: "ART001".to_string(),
                    quantity: "5".to_string(),
                },
                PosRow {
                    position_nr: "2".to_string(),
                    article_code: "ART002".to_string(),
                    quantity: "3".to_string(),
                },
            ],
        };

        let serialized = serde_support::to_dta_string(&row, identifier).unwrap();
        assert_eq!(
            serialized,
            r#"þVARTþ0þSKZþBELþUEBERþNþSTAMMKALKþJþaaþ809460 þacþClaas 
þVARTþ0þSKZþPOSþUEBERþNþaaþ1 þabþART001 þacþ5 
þVARTþ0þSKZþPOSþUEBERþNþaaþ2 þabþART002 þacþ3 
"#
        );
    }

    #[cfg(all(feature = "std", feature = "io"))]
    #[test]
    fn test_writer() {
        let mut buffer = Vec::new();
        let mut writer = DtaWriter::new(&mut buffer);

        let identifier = DynamicRecordIdentifier::new("ADR");
        let row = DtaRowBuilder::new(identifier)
            .param("STAMMKALK", "J")
            .unwrap()
            .param("LANDKUNDA", "J")
            .unwrap()
            .data("aa", "809460")
            .data("ac", "Claas")
            .data("ad", "Elke")
            .build()
            .unwrap();

        writer.write_row(&row).unwrap();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("þVARTþ0þSKZþADRþ"));
    }

    #[test]
    fn test_parsing_integration() {
        let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
        
        // Test dynamic parsing
        let dynamic_row = parse_dta_string(dta_string).unwrap();
        assert_eq!(dynamic_row.identifier.skz(), "ADR");
        assert_eq!(dynamic_row.params.get("STAMMKALK"), Some(&"J".to_string()));
        
        // Test typed parsing
        let typed_row = parse_dta_string_with_identifier::<identifiers::AddressIdentifier>(dta_string).unwrap();
        assert_eq!(typed_row.params.get("STAMMKALK"), Some(&"J".to_string()));
        
        // Test DtaRow::from_dta_string
        let row_from_method = DtaRow::<identifiers::AddressIdentifier>::from_dta_string(dta_string).unwrap();
        assert_eq!(row_from_method.params.get("STAMMKALK"), Some(&"J".to_string()));
    }

    #[test]
    fn test_round_trip_parsing() {
        let identifier = DynamicRecordIdentifier::new("ADR");
        let original_row = DtaRowBuilder::new(identifier)
            .param("STAMMKALK", "J")
            .unwrap()
            .param("LANDKUNDA", "J")
            .unwrap()
            .data("aa", "809460")
            .data("ac", "Claas")
            .data("ad", "Elke")
            .build()
            .unwrap();

        let dta_string = original_row.to_dta_string().unwrap();
        let parsed_row = parse_dta_string(&dta_string).unwrap();
        
        // Should produce equivalent DTA string
        assert_eq!(dta_string, parsed_row.to_dta_string().unwrap());
    }
}
