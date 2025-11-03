# bwdta

A Rust library for working with the DTA (Data Transfer) format used by ERPSuite (formerly BüroWARE), a German ERP software by SoftENGINE. This format is used for importing and exporting structured business data.

## Features

- **Builder pattern** for constructing DTA rows with type-safe record identifiers
- **Serde support** for serializing Rust structs directly to DTA format
- **no_std compatible** for embedded and resource-constrained environments
- **Predefined identifiers** for common record types (ADR, ART, BEL, POS, etc.)
- **Dynamic identifiers** for custom record types
- **Parameter validation** to ensure required fields are present

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bwdta = "0.1.0"
```

### Feature Flags

- **`std`** (default): Enable standard library support
- **`io`** (default): Enable I/O utilities like `DtaWriter` (requires `std`)
- **`serde`**: Enable serde serialization support

To use without default features:

```toml
[dependencies]
bwdta = { version = "0.1.0", default-features = false }
```

## Usage

### Basic Usage with Builder Pattern

```rust
use bwdta::{DtaRowBuilder, DynamicRecordIdentifier};

let identifier = DynamicRecordIdentifier::new("ADR");
let row = DtaRowBuilder::new(identifier)
    .param("STAMMKALK", "J")?
    .param("LANDKUNDA", "J")?
    .data("aa", "809460")
    .data("ac", "Claas")
    .data("ad", "Elke")
    .build()?;

let output = row.to_dta_string()?;
// Output: þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n
```

### Using Predefined Identifiers

```rust
use bwdta::{DtaRowBuilder, identifiers::AddressIdentifier};

let identifier = AddressIdentifier;
let row = DtaRowBuilder::new(identifier)
    .param("LANDKUNDA", "J")?
    .data("aa", "123456")
    .build()?;
```

Predefined identifiers provide compile-time validation of parameters specific to each record type.

### Serde Serialization

With the `serde` feature enabled:

```rust
use serde::Serialize;
use bwdta::{DynamicRecordIdentifier, serde_support::to_dta_string};

#[derive(Serialize)]
struct Address {
    #[serde(rename = "STAMMKALK")]
    stammkalk: String,
    #[serde(rename = "$LANDKUNDA$")]  // $ prefix/suffix marks parameters
    landkunda: String,
    #[serde(rename = "aa")]
    customer_nr: String,
    #[serde(rename = "ac")]
    first_name: String,
    #[serde(rename = "ad")]
    last_name: String,
}

let address = Address {
    stammkalk: "J".to_string(),
    landkunda: "J".to_string(),
    customer_nr: "809460".to_string(),
    first_name: "Claas".to_string(),
    last_name: "Elke".to_string(),
};

let identifier = DynamicRecordIdentifier::new("ADR");
let output = to_dta_string(&address, identifier)?;
```

### Nested Records (Sequences)

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Order {
    #[serde(rename = "STAMMKALK")]
    stammkalk: String,
    #[serde(rename = "aa")]
    order_nr: String,
    
    #[serde(rename = "POS")]  // Nested sequence with record type
    positions: Vec<Position>,
}

#[derive(Serialize)]
struct Position {
    #[serde(rename = "aa")]
    position_nr: String,
    #[serde(rename = "ab")]
    article_code: String,
    #[serde(rename = "ac")]
    quantity: String,
}
```

### Writing to Files

With the `io` feature enabled:

```rust
use std::fs::File;
use bwdta::{DtaWriter, DtaRowBuilder, DynamicRecordIdentifier};

let file = File::create("output.dta")?;
let mut writer = DtaWriter::new(file);

let identifier = DynamicRecordIdentifier::new("ADR");
let row = DtaRowBuilder::new(identifier)
    .param("STAMMKALK", "J")?
    .data("aa", "123456")
    .build()?;

writer.write_row(&row)?;
```

## Record Identifiers

The library provides several predefined record identifiers:

- **`AddressIdentifier`** (ADR): Customer/address records
- **`AddressProductIdentifier`** (ADA): Address-product relationships
- **`ProductIdentifier`** (ART): Product/article records
- **`DeliveryAddressIdentifier`** (LFA): Delivery address records
- **`OrderIdentifier`** (BEL): Order/document records
- **`OrderNoteIdentifier`** (BNOT/BNOTN/BNOTV): Order notes
- **`OrderPositionIdentifier`** (POS): Order line items

Each predefined identifier includes:
- Valid parameter lists (required and optional)
- Parameter ordering rules
- Compile-time validation

Use `DynamicRecordIdentifier` for custom record types without validation.

## Format Details

The DTA format uses:
- **`þ`** (thorn character, U+00FE) as field separator
- **`SKZ`** (Satzkennzeichen/Record Identifier) to identify record types
- **Parameters** prefixed with names (e.g., `STAMMKALK`, `LANDKUNDA`)
- **Data fields** with short codes (e.g., `aa`, `ab`, `ac`, `n12`, `a01`)
- **Newline** terminator for each record
- **Windows-1252 encoding** (Codepage 1252) for file storage

### Encoding Considerations

DTA files are typically stored with **Windows-1252** (CP1252) encoding, not UTF-8. When writing to files, you should encode the output appropriately:

```rust
use std::fs::File;
use std::io::Write;
use encoding_rs::WINDOWS_1252;

let dta_string = row.to_dta_string()?;
let (encoded, _, _) = WINDOWS_1252.encode(&dta_string);

let mut file = File::create("output.dta")?;
file.write_all(&encoded)?;
```

**Note**: The library generates UTF-8 strings internally. You are responsible for encoding to Windows-1252 when writing to files if required by your target system.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
