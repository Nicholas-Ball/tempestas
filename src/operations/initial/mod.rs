pub mod grib_source;

pub trait IntialOperation {
    fn initial_operation(&self) -> ndarray::Array2<f64>;
}
