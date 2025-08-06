pub struct Model {
    pub name: String,
    pub version: String,
    pub description: String,
}

impl Model {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Model {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!("Model Name: {}, Version: {}, Description: {}", self.name, self.version, self.description)
    }
}
