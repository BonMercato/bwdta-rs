//! Demonstrates DTA file parsing functionality.

use bwdta::{
    parse_dta_string, parse_dta_string_with_identifier, parse_dta_rows,
    identifiers::AddressIdentifier, DtaRow, RecordIdentifier
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example DTA string
    let dta_string = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþLANDKUNDAþJþaaþ809460 þacþClaas þadþElke \n";
    
    println!("=== DTA Parsing Demo ===\n");
    println!("Original DTA string:");
    println!("{}", dta_string);
    
    // Parse with dynamic identifier
    println!("1. Parsing with dynamic identifier:");
    let dynamic_row = parse_dta_string(dta_string)?;
    println!("   SKZ: {}", dynamic_row.identifier().skz());
    println!("   Parameters: {:?}", dynamic_row.params);
    println!("   Data fields: {:?}", dynamic_row.data_fields);
    
    // Parse with typed identifier
    println!("\n2. Parsing with typed identifier (AddressIdentifier):");
    let typed_row = parse_dta_string_with_identifier::<AddressIdentifier>(dta_string)?;
    println!("   SKZ: {}", typed_row.identifier().skz());
    println!("   Parameters: {:?}", typed_row.params);
    println!("   Data fields: {:?}", typed_row.data_fields);
    
    // Parse using DtaRow::from_dta_string method
    println!("\n3. Parsing using DtaRow::from_dta_string method:");
    let method_row = DtaRow::<AddressIdentifier>::from_dta_string(dta_string)?;
    println!("   SKZ: {}", method_row.identifier().skz());
    println!("   Parameters: {:?}", method_row.params);
    
    // Demonstrate round-trip
    println!("\n4. Round-trip test:");
    let serialized = method_row.to_dta_string()?;
    println!("   Serialized matches original: {}", dta_string == serialized);
    
    // Parse multiple rows
    println!("\n5. Parsing multiple rows:");
    let multi_dta = "þVARTþ0þSKZþADRþUEBERþNþSTAMMKALKþJþaaþ809460 \n\
                     þVARTþ0þSKZþPOSþUEBERþNþaaþ1 þabþART001 \n\
                     þVARTþ0þSKZþPOSþUEBERþNþaaþ2 þabþART002 \n";
    let rows = parse_dta_rows(multi_dta)?;
    println!("   Parsed {} rows", rows.len());
    for (i, row) in rows.iter().enumerate() {
        println!("   Row {}: SKZ={}, {} data fields", i + 1, row.identifier().skz(), row.data_fields.len());
    }
    
    // Demonstrate arbitrary parameter order parsing
    println!("\n6. Arbitrary parameter order:");
    let arbitrary_dta = "þVARTþ1þSKZþADRþUEBERþJþLANDKUNDAþNþSTAMMKALKþJþaaþ12345 þabþTest \n";
    let arbitrary_row = parse_dta_string(arbitrary_dta)?;
    println!("   Parameters: {:?}", arbitrary_row.params);
    println!("   Data fields: {:?}", arbitrary_row.data_fields);
    
    // Demonstrate VART not being first
    println!("\n7. VART in different positions:");
    let vart_middle = "þSKZþADRþVARTþ2þUEBERþYþSTAMMKALKþJþaaþ99999 \n";
    let vart_middle_row = parse_dta_string(vart_middle)?;
    println!("   VART in middle: VART={}, SKZ={}", 
             vart_middle_row.params.get("VART").unwrap_or(&"missing".to_string()),
             vart_middle_row.identifier().skz());
    
    let vart_last = "þUEBERþNþSKZþADRþSTAMMKALKþJþVARTþ3þaaþ88888 \n";
    let vart_last_row = parse_dta_string(vart_last)?;
    println!("   VART at end: VART={}, UEBER={}", 
             vart_last_row.params.get("VART").unwrap_or(&"missing".to_string()),
             vart_last_row.params.get("UEBER").unwrap_or(&"missing".to_string()));
    
    Ok(())
}
