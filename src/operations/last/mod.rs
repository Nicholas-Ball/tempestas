pub mod grib_sink;

pub trait FinalOperation {
    fn final_operation(&self, input: &ndarray::Array2<f64>);
}
