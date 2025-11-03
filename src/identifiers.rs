//! Record identifiers for different DTA record types.
//!
//! This module provides the [`RecordIdentifier`] trait and various implementations
//! for common record types used in the DTA format.

#![allow(clippy::module_name_repetitions)]

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeSet as HashSet;
#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(feature = "std")]
use std::collections::HashSet;
#[cfg(feature = "std")]
use std::string::{String, ToString};

/// Default required parameters (none by default).
pub static DEFAULT_REQUIRED_PARAMS: [&str; 0] = [];
/// Default optional parameters available for all record types.
pub static DEFAULT_OPTIONAL_PARAMS: [&str; 9] = [
    "PROT",
    "STAMMKALK",
    "AENDERV",
    "ALLEALPHA",
    "READONLY",
    "ANSIASCII",
    "UTF8_ANSI",
    "HTTPREQUEST",
    "UEBER",
];
/// Default parameter ordering for output.
pub static DEFAULT_PARAM_ORDER: [&str; 9] = [
    "UEBER",
    "PROT",
    "STAMMKALK",
    "AENDERV",
    "ALLEALPHA",
    "READONLY",
    "ANSIASCII",
    "UTF8_ANSI",
    "HTTPREQUEST",
];

/// Trait for record identifiers that define the structure and validation rules for DTA records.
///
/// This trait allows different record types to have their own parameter validation rules
/// and default parameter orders while maintaining a common interface for serialization
/// and deserialization.
pub trait RecordIdentifier: Clone {
    /// Whether this identifier type validates parameters.
    fn check_params() -> bool;
    /// Returns the set of required parameters for this record type.
    fn required_params() -> HashSet<String>;
    /// Returns the set of optional parameters for this record type.
    fn optional_params() -> HashSet<String> {
        DEFAULT_OPTIONAL_PARAMS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    /// Returns the set of parameters that have a defined ordering.
    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER.iter().map(|s| s.to_string()).collect()
    }
    /// Returns the SKZ (Satzkennzeichen/Record Identifier) for this record type.
    fn skz(&self) -> String;
    /// Creates an identifier instance from an SKZ string.
    fn from_skz(skz: &str) -> Self;
}

/// A dynamic record identifier that accepts any SKZ without parameter validation.
///
/// Use this when you need to work with custom or unknown record types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicRecordIdentifier {
    skz: String,
}

impl DynamicRecordIdentifier {
    /// Creates a new dynamic record identifier with the given SKZ.
    #[must_use]
    pub fn new(skz: &str) -> Self {
        Self {
            skz: skz.to_string(),
        }
    }
}

impl Default for DynamicRecordIdentifier {
    fn default() -> Self {
        Self::new("UNKNOWN")
    }
}

impl RecordIdentifier for DynamicRecordIdentifier {
    fn check_params() -> bool {
        false
    }

    fn required_params() -> HashSet<String> {
        DEFAULT_REQUIRED_PARAMS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn optional_params() -> HashSet<String> {
        DEFAULT_OPTIONAL_PARAMS
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER.iter().map(|s| s.to_string()).collect()
    }

    fn skz(&self) -> String {
        self.skz.clone()
    }

    fn from_skz(skz: &str) -> Self {
        Self::new(skz)
    }
}

/// Record identifier for address records (ADR).
///
/// Used for customer and address data imports.
#[derive(Clone)]
pub struct AddressIdentifier;

impl RecordIdentifier for AddressIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        ["STAMMKALK", "LANDKUNDA"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn skz(&self) -> String {
        "ADR".to_string()
    }

    fn optional_params() -> HashSet<String> {
        [
            "LANDKUNDA",
            "LOADEMAIL",
            "NURNEUANLAGE",
            "CODEPRF",
            "NEUE_ADR",
            "ADD_ADRSELPOOL",
            "ADD_ANP",
            "WFLSCRIPT_NACHIMPORT_STARTEN",
            // TODO: Add functionality that allows parameters to depend on other parameters
            // e.g. "SCRIPT_NR_NACH" needs to be set if "WFLSCRIPT_NACHIMPORT_STARTEN" is set
            "SCRIPT_NR_NACH",
            "SCRIPT_UEBERGABE",
            "ADRKREIS",
        ]
        .iter()
        .chain(DEFAULT_OPTIONAL_PARAMS.iter())
        .map(|s| s.to_string())
        .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&[
                "LANDKUNDA",
                "LOADEMAIL",
                "NURNEUANLAGE",
                "CODEPRF",
                "NEUE_ADR",
                "ADD_ADRSELPOOL",
                "ADD_ANP",
                "WFLSCRIPT_NACHIMPORT_STARTEN",
                // TODO: Add functionality that allows parameters to depend on other parameters
                // e.g. "SCRIPT_NR_NACH" needs to be set if "WFLSCRIPT_NACHIMPORT_STARTEN" is set
                "SCRIPT_NR_NACH",
                "SCRIPT_UEBERGABE",
                "ADRKREIS",
            ])
            .map(|s| s.to_string())
            .collect()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

/// Record identifier for address-product relationship records (ADA).
///
/// Used for linking products to specific addresses.
#[derive(Clone)]
pub struct AddressProductIdentifier;

impl RecordIdentifier for AddressProductIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        ["STAMMKALK", "LANDKUNDA"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn optional_params() -> HashSet<String> {
        [
            "EANCODE",
            "BESTELLNR_UMSETZEN",
            "ADD_ADASELPOOL",
            "MP_PROTOKOLLIERUNG",
            "PRUEFE_ARTNR",
        ]
        .iter()
        .chain(DEFAULT_OPTIONAL_PARAMS.iter())
        .map(|s| s.to_string())
        .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&[
                "EANCODE",
                "BESTELLNR_UMSETZEN",
                "ADD_ADASELPOOL",
                "MP_PROTOKOLLIERUNG",
                "PRUEFE_ARTNR",
            ])
            .map(|s| s.to_string())
            .collect()
    }

    fn skz(&self) -> String {
        "ADA".to_string()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

/// Record identifier for product/article records (ART).
///
/// Used for product master data imports.
#[derive(Clone)]
pub struct ProductIdentifier;

impl RecordIdentifier for ProductIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        HashSet::new() // No required params for ART
    }

    fn skz(&self) -> String {
        "ART".to_string()
    }

    fn optional_params() -> HashSet<String> {
        [
            "EANCODE",
            "BESTELLNR_UMSETZEN",
            "ARTTEXT",
            "MP_PRODID",
            "MP_ID",
            "MP_PROTOKOLLIERUNG",
            "MP_UPLOADDATUMSETZEN",
            "EKUPDATE",
            "STK_A01_PREIS",
            "STK_SUBARTIKEL",
            "ADD_ARTSELPOOL",
            "POSLTXTUEBER",
            "WFLSCRIPT_NACHIMPORT_STARTEN",
            "SCRIPT_NR_NACH",
            "SCRIPT_UEBERGABE",
        ]
        .iter()
        .chain(DEFAULT_OPTIONAL_PARAMS.iter())
        .map(|s| s.to_string())
        .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&[
                "EANCODE",
                "BESTELLNR_UMSETZEN",
                "ARTTEXT",
                "MP_PRODID",
                "MP_ID",
                "MP_PROTOKOLLIERUNG",
                "MP_UPLOADDATUMSETZEN",
                "EKUPDATE",
                "STK_A01_PREIS",
                "STK_SUBARTIKEL",
                "ADD_ARTSELPOOL",
                "POSLTXTUEBER",
                "WFLSCRIPT_NACHIMPORT_STARTEN",
                "SCRIPT_NR_NACH",
                "SCRIPT_UEBERGABE",
            ])
            .map(|s| s.to_string())
            .collect()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

/// Record identifier for delivery address records (LFA).
///
/// Used for supplier/delivery address data.
#[derive(Clone)]
pub struct DeliveryAddressIdentifier;

impl RecordIdentifier for DeliveryAddressIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        HashSet::new() // No required params for LFA
    }

    fn skz(&self) -> String {
        "LFA".to_string()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

/// Record identifier for order/document records (BEL).
///
/// Used for order headers and document imports.
#[derive(Clone)]
pub struct OrderIdentifier;

impl RecordIdentifier for OrderIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        HashSet::new() // No required params for BEL
    }

    fn skz(&self) -> String {
        "BEL".to_string()
    }

    fn optional_params() -> HashSet<String> {
        [
            "BELDATERG",
            "BELGESNULL",
            "BELGESSUM",
            "BELSUMMENBER",
            "VERSANDBER",
            "ST_KALK",
            "DEL_BELSELPOOL",
            "ADD_BELSELPOOL",
            "NUR_WFLSCRIPT_STARTEN",
            "SCRIPT_NR",
            "WFLSCRIPT_NACHIMPORT_STARTEN",
            "SCRIPT_NR_NACH",
            "SCRIPT_UEBERGABE",
            "ZKO_BERECHNEN",
            "NEUERBELEG",
        ]
        .iter()
        .chain(DEFAULT_OPTIONAL_PARAMS.iter())
        .map(|s| s.to_string())
        .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&[
                "BELDATERG",
                "BELGESNULL",
                "BELGESSUM",
                "BELSUMMENBER",
                "VERSANDBER",
                "ST_KALK",
                "DEL_BELSELPOOL",
                "ADD_BELSELPOOL",
                "NUR_WFLSCRIPT_STARTEN",
                "SCRIPT_NR",
                "WFLSCRIPT_NACHIMPORT_STARTEN",
                "SCRIPT_NR_NACH",
                "SCRIPT_UEBERGABE",
                "ZKO_BERECHNEN",
                "NEUERBELEG",
            ])
            .map(|s| s.to_string())
            .collect()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

/// Record identifier for order note records.
///
/// Supports different note types: BNOT, BNOTN, and BNOTV.
#[derive(Clone)]
pub enum OrderNoteIdentifier {
    BNOT,
    BNOTN,
    BNOTV,
}

impl RecordIdentifier for OrderNoteIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        HashSet::new() // No required params for BNOT
    }

    fn skz(&self) -> String {
        match self {
            OrderNoteIdentifier::BNOT => "BNOT".to_string(),
            OrderNoteIdentifier::BNOTN => "BNOTN".to_string(),
            OrderNoteIdentifier::BNOTV => "BNOTV".to_string(),
        }
    }

    fn optional_params() -> HashSet<String> {
        ["NOTIZART"]
            .iter()
            .chain(DEFAULT_OPTIONAL_PARAMS.iter())
            .map(|s| s.to_string())
            .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&["NOTIZART"])
            .map(|s| s.to_string())
            .collect()
    }

    fn from_skz(skz: &str) -> Self {
        match skz {
            "BNOT" => OrderNoteIdentifier::BNOT,
            "BNOTN" => OrderNoteIdentifier::BNOTN,
            "BNOTV" => OrderNoteIdentifier::BNOTV,
            _ => OrderNoteIdentifier::BNOT, // Default fallback
        }
    }
}

/// Record identifier for order position/line item records (POS).
///
/// Used for order line items within an order.
#[derive(Clone)]
pub struct OrderPositionIdentifier;

impl RecordIdentifier for OrderPositionIdentifier {
    fn check_params() -> bool {
        true
    }

    fn required_params() -> HashSet<String> {
        HashSet::new() // No required params for POS
    }

    fn skz(&self) -> String {
        "POS".to_string()
    }

    fn optional_params() -> HashSet<String> {
        [
            "FOSTAND",
            "FOMENGE",
            "FOEAN",
            "PREISBER",
            "POSDATERG",
            "EAN",
            "POSITIONSVERBUCHUNG",
            "POSNR",
            "BELPOSTITEL",
            "POSLTXTUEBER",
            "CHAAUTO",
            "SERAUTO",
            "IMMERUEBER",
            "ZUEBER",
            "FORMEL_1029",
            "FOLGE_ARTIKEL",
            "POSDATEN_SICHERN",
            "POSDATEN_LADEN",
            "SCRIPT_NR",
            "ARTKIT_AUFLOESEN",
            "HSTKL_AUFLOESUNG",
            "AUF_HST_PRUEFEN",
            "ZWISCHENSUMMEN",
            "ALTERNATIVARTIKEL",
        ]
        .iter()
        .chain(DEFAULT_OPTIONAL_PARAMS.iter())
        .map(|s| s.to_string())
        .collect()
    }

    fn param_order() -> HashSet<String> {
        DEFAULT_PARAM_ORDER
            .iter()
            .chain(&[
                "FOSTAND",
                "FOMENGE",
                "FOEAN",
                "PREISBER",
                "POSDATERG",
                "EAN",
                "POSITIONSVERBUCHUNG",
                "POSNR",
                "BELPOSTITEL",
                "POSLTXTUEBER",
                "CHAAUTO",
                "SERAUTO",
                "IMMERUEBER",
                "ZUEBER",
                "FORMEL_1029",
                "FOLGE_ARTIKEL",
                "POSDATEN_SICHERN",
                "POSDATEN_LADEN",
                "SCRIPT_NR",
                "ARTKIT_AUFLOESEN",
                "HSTKL_AUFLOESUNG",
                "AUF_HST_PRUEFEN",
                "ZWISCHENSUMMEN",
                "ALTERNATIVARTIKEL",
            ])
            .map(|s| s.to_string())
            .collect()
    }

    fn from_skz(_skz: &str) -> Self {
        Self
    }
}

impl Default for AddressIdentifier {
    fn default() -> Self {
        Self
    }
}

impl Default for AddressProductIdentifier {
    fn default() -> Self {
        Self
    }
}

impl Default for ProductIdentifier {
    fn default() -> Self {
        Self
    }
}

impl Default for DeliveryAddressIdentifier {
    fn default() -> Self {
        Self
    }
}

impl Default for OrderIdentifier {
    fn default() -> Self {
        Self
    }
}

impl Default for OrderNoteIdentifier {
    fn default() -> Self {
        OrderNoteIdentifier::BNOT
    }
}

impl Default for OrderPositionIdentifier {
    fn default() -> Self {
        Self
    }
}
