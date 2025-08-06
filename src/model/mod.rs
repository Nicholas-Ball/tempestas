use crate::{ operations::{initial::IntialOperation, Operation, last::FinalOperation}};

pub struct Model {
    pub name: String,
    pub version: String,
    pub description: String,
    pub intial_operation: Option<Box<dyn IntialOperation>>,
    pub operations: Vec<Box<dyn Operation>>,
    pub final_operation: Option<Box<dyn FinalOperation>>,
}

impl Model {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Model {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            intial_operation: None,
            operations: Vec::new(),
            final_operation: None,
        }
    }

    pub fn add_initial_operation(&mut self, operation: Box<dyn IntialOperation>) {
        self.intial_operation = Some(operation);
    }

    pub fn add_final_operation(&mut self, operation: Box<dyn FinalOperation>) {
        self.final_operation = Some(operation);
    }

    pub fn add_operation(&mut self, operation: Box<dyn Operation>) {
        self.operations.push(operation);
    }

    pub fn run(&self) {
        if let Some(ref initial_op) = self.intial_operation {
            let mut initial_result = initial_op.initial_operation();
            println!("Initial operation result: {:?}", initial_result);

            for operation in &self.operations {
                initial_result = operation.execute(&initial_result);
            }

            if let Some(ref final_op) = self.final_operation {
                final_op.final_operation(&initial_result);
            }
        }
    }

    pub fn display(&self) -> String {
        format!("Model Name: {}, Version: {}, Description: {}", self.name, self.version, self.description)
    }
}
