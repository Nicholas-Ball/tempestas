use crate::operations::initial::IntialOperation;

pub struct GribSource {
    pub(crate) source: String,
}

impl GribSource {
    pub fn new(source: &str) -> Self {
        GribSource {
            source: source.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!("Grib Source: {}", self.source)
    }
}

impl std::fmt::Display for GribSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl IntialOperation for GribSource {
    fn initial_operation(&self) -> ndarray::Array2<f64> {
        // For demonstration purposes, we return a 2x2 matrix filled with zeros.
        // In a real implementation, this would likely involve reading from the GRIB source.
        ndarray::Array2::<f64>::zeros((2, 2))
    }
}
