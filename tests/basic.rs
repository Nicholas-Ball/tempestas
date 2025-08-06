use tempestas::operations::last::grib_sink::GribSink;
use tempestas::{model::Model, operations::initial::grib_source};
use tempestas::operations::fundamental::matrix_mul::MarixMul;

#[test]
pub fn test_basic() {
    let mut m = Model::new("Test Model", "1.0", "This is a test model.");

    let gsource = grib_source::GribSource::new("source.grib");
    let mm = MarixMul::new(ndarray::Array2::ones((2, 2)));
    let gsink = GribSink::new("sink.grib");

    m.add_initial_operation(Box::new(gsource));
    m.add_operation(Box::new(mm));
    m.add_final_operation(Box::new(gsink));

    m.run();
}
