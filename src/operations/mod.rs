pub mod initial;
pub mod last;
pub mod fundamental;

pub trait Operation {
    fn execute(&self, input: &ndarray::Array2<f64>) -> ndarray::Array2<f64>;
}
