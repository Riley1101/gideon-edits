#[derive(Debug, Default)]
pub struct Plugin {
    status: String,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            status: String::from("Loaded"),
        }
    }

    fn execute() -> String {
        String::new()
    }
}
