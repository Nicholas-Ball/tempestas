use crate::operations::last::FinalOperation;

pub struct GribSink {
    pub grib_file: String,
}

impl GribSink {
    pub fn new(grib_file: &str) -> Self {
        GribSink { grib_file: grib_file.to_string() }
    }

    pub fn write_data(&self, data: &ndarray::Array2<f64>) -> Result<(), std::io::Error> {
        // Here you would implement the logic to write the data to a GRIB file.
        // This is a placeholder implementation.
        println!("Writing data to GRIB file: {}", self.grib_file);
        Ok(())
    }
}

impl FinalOperation for GribSink {
    fn final_operation(&self, input: &ndarray::Array2<f64>) {
        if let Err(e) = self.write_data(input) {
            eprintln!("Error writing data to GRIB file: {}", e);
        } else {
            println!("Data successfully written to GRIB file: {}", self.grib_file);
        }
    }
}
