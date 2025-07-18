/// Represents a GRIB file.
pub mod discipline;
pub mod indicator;

use std::io::Read;

use anyhow::Result;
use rkyv::{Archive, Deserialize, Serialize, rancor::Error};

use crate::grib::{discipline::Discipline, indicator::ArchivedIndicator};

/// A GribFile is a wrapper around an archived GRIB file and its path.
#[derive(Archive, Deserialize, Serialize, Clone)]
pub struct GribFile<'a> {
    /// The archived GRIB indicator of the file.
    indicator: &'a ArchivedIndicator,
    /// The path to the GRIB file on disk.
    path: String,
}

impl<'a> GribFile<'a> {
    /// Returns a reference to the archived GRIB indicator.
    ///
    /// # Returns
    /// A reference to the `ArchivedIndicator`.
    pub fn get_indicator(&self) -> &ArchivedIndicator {
        self.indicator
    }

    /// Returns the path to the GRIB file.
    ///
    /// # Returns
    /// A string slice of the path.
    pub fn get_path(&self) -> &str {
        &self.path
    }

    /// Returns the edition number of the GRIB file.
    ///
    /// # Returns
    /// The edition number as a `u8`.
    pub fn get_edition(&self) -> u8 {
        self.indicator.edition
    }

    /// Returns the discipline of the GRIB file.
    ///
    /// # Returns
    /// A `Discipline` enum value.
    pub fn get_discipline(&self) -> Discipline {
        Discipline::from_u8(self.indicator.discipline).unwrap()
    }

    /// Returns the length of the GRIB file in bytes.
    ///
    /// # Returns
    /// The length as a `u64`.
    pub fn get_length(&self) -> u64 {
        self.indicator.length.to_native()
    }
}

/// Opens and reads a GRIB file into memory, returning a `GribFile`.
///
/// # Arguments
/// * `path` - A string slice that holds the path to the GRIB file.
/// * `buffer` - A mutable byte array buffer used for reading the file's contents.
///
/// # Returns
/// A `Result` containing either the parsed `GribFile` or an error.
pub fn open<'a>(path: &str, buffer: &'a mut [u8]) -> Result<GribFile<'a>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);

    // Read the first 16 bytes to check the magic number and other metadata
    reader.read_exact(buffer)?;

    // Check the magic number
    if buffer.len() < 16 || buffer[0..4] != [0x47, 0x52, 0x49, 0x42] {
        return Err(anyhow::anyhow!("Invalid GRIB file: magic number mismatch"));
    }

    let indicator = rkyv::access::<ArchivedIndicator, Error>(buffer)?;
    Ok(GribFile {
        indicator,
        path: path.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let mut buffer = [0u8; 16];
        let result = open("tests/20250717_021449_GFS_P25_18.grb2", &mut buffer);
        assert!(result.is_ok());

        dbg!(result.unwrap().get_length());
    }
}
