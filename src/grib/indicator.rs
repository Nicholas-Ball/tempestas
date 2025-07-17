use rkyv::{Archive, Deserialize, Serialize};

const MAGIC: u32 = 0x4742494D; // 'GRIB' in ASCII

/// Represents the GRIB Indicator section.
///
/// This struct holds essential metadata about a GRIB file.
#[derive(Archive, Deserialize, Serialize)]
pub struct Indicator {
    /// Magic number to identify the GRIB file format.
    pub(crate) magic: u32,

    /// Reserved field for future use.
    pub(crate) reserved: u16,

    /// Discipline of the data in the GRIB file (e.g., meteorology).
    pub(crate) discipline: u8,

    /// Edition of the GRIB format.
    pub(crate) edition: u8,

    /// Length of the Indicator section in bytes.
    pub(crate) length: u64,
}
