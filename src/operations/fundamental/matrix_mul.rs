use crate::operations::Operation;

pub struct MarixMul {
    pub matrix: ndarray::Array2<f64>,
}

impl MarixMul {
    pub fn new(matrix: ndarray::Array2<f64>) -> Self {
        MarixMul { matrix }
    }

    pub fn display(&self) -> String {
        format!("MatrixMul: \n{}", self.matrix)
    }
}

impl Operation for MarixMul {
    fn execute(&self, input: &ndarray::Array2<f64>) -> ndarray::Array2<f64> {
        // Perform matrix multiplication
        input.dot(&self.matrix)
    }
}
