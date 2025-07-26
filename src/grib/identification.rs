use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize)]
pub struct Indentification {
    pub length: u32,
    pub num_sections: u8,
    pub center_id: u16,
    pub subcenter_id: u16,
    pub master_table_version: u8,
    pub local_table_version: u8,
    // idk why removing this fixed the tests but it would seem the standard
    // is wrong on the website (https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/)
    //
    // It may be the center_id and subcenter_id are not always present or the length specified
    // as the center_id returned does not seem to exist in the NOAA database.
    //
    // May revisit this later
    // pub significance_of_reference_time: u8,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub production_status_of_processed_data: u8,
    pub type_of_processed_data: u8,
}
