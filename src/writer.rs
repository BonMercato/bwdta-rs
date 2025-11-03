//! Writer utilities for writing DTA rows to I/O streams.
//!
//! This module provides [`DtaWriter`] for writing DTA records to any type
//! that implements [`std::io::Write`].

#[cfg(feature = "std")]
use std::io::Write;

use crate::{error::DtaError, identifiers::RecordIdentifier, row::DtaRow};

/// A writer for outputting DTA rows to an I/O stream.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use bwdta::{DtaWriter, DtaRowBuilder, DynamicRecordIdentifier};
///
/// let mut buffer = Vec::new();
/// let mut writer = DtaWriter::new(&mut buffer);
///
/// let identifier = DynamicRecordIdentifier::new("ADR");
/// let row = DtaRowBuilder::new(identifier)
///     .param("STAMMKALK", "J")?
///     .data("aa", "809460")
///     .build()?;
///
/// writer.write_row(&row)?;
/// # Ok::<(), bwdta::DtaError>(())
/// ```
#[cfg(all(feature = "std", feature = "io"))]
pub struct DtaWriter<W: Write> {
    writer: W,
}

#[cfg(all(feature = "std", feature = "io"))]
impl<W: Write> DtaWriter<W> {
    /// Creates a new DTA writer that writes to the given writer.
    #[must_use]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Writes a DTA row to the underlying writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails or if the row fails validation.
    pub fn write_row<T: RecordIdentifier>(&mut self, row: &DtaRow<T>) -> Result<(), DtaError> {
        let line = row.to_dta_string()?;
        write!(self.writer, "{}", line)?;
        Ok(())
    }

    /// Consumes this writer and returns the underlying writer.
    #[must_use]
    pub fn into_inner(self) -> W {
        self.writer
    }
}
