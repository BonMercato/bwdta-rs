//! Demonstrates Serde serialization and deserialization for DTA format.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use bwdta::{from_dta_string, from_dta_string_with_identifier, to_dta_string, DynamicRecordIdentifier};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Address {
    #[serde(rename = "STAMMKALK")]
    stammkalk: String,
    #[serde(rename = "LANDKUNDA")]
    landkunda: String,
    #[serde(rename = "aa")]
    customer_nr: String,
    #[serde(rename = "ac")]
    customer_name: String,
    #[serde(rename = "ad")]
    customer_contact: String,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Product {
    #[serde(rename = "aa")]
    position_nr: String,
    #[serde(rename = "ab")]
    article_code: String,
    #[serde(rename = "ac")]
    quantity: String,
}

#[cfg(feature = "serde")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Serde DTA Demo ===\n");

    // 1. Serialization example
    println!("1. Serialization:");
    let address = Address {
        stammkalk: "J".to_string(),
        landkunda: "J".to_string(),
        customer_nr: "809460".to_string(),
        customer_name: "Claas".to_string(),
        customer_contact: "Elke".to_string(),
    };

    let identifier = DynamicRecordIdentifier::new("ADR");
    let dta_string = to_dta_string(&address, identifier)?;
    println!("   Original struct: {:?}", address);
    println!("   Serialized DTA: {}", dta_string);

    // 2. Deserialization example
    println!("\n2. Deserialization:");
    let dta_input = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
    let deserialized: Address = from_dta_string::<DynamicRecordIdentifier, _>(dta_input)?;
    println!("   DTA input: {}", dta_input);
    println!("   Deserialized struct: {:?}", deserialized);

    // 3. Round-trip test
    println!("\n3. Round-trip test:");
    let product = Product {
        position_nr: "1".to_string(),
        article_code: "ART001".to_string(),
        quantity: "5".to_string(),
    };

    let product_identifier = DynamicRecordIdentifier::new("POS");
    let serialized_product = to_dta_string(&product, product_identifier.clone())?;
    let deserialized_product: Product = from_dta_string_with_identifier(&serialized_product, product_identifier)?;
    
    println!("   Original: {:?}", product);
    println!("   Serialized: {}", serialized_product);
    println!("   Deserialized: {:?}", deserialized_product);
    println!("   Round-trip successful: {}", product == deserialized_product);

    // 4. Arbitrary parameter order
    println!("\n4. Arbitrary parameter order:");
    let arbitrary_dta = "þSTAMMKALKþJþVARTþ1þSKZþADRþUEBERþYþLANDKUNDAþNþaaþ12345 þacþTest þadþContact \n";
    let arbitrary_deserialized: Address = from_dta_string::<DynamicRecordIdentifier, _>(arbitrary_dta)?;
    println!("   Arbitrary order DTA: {}", arbitrary_dta);
    println!("   Deserialized: {:?}", arbitrary_deserialized);

    Ok(())
}

#[cfg(not(feature = "serde"))]
fn main() {
    println!("This example requires the 'serde' feature to be enabled.");
    println!("Run with: cargo run --example serde_demo --features serde");
}
