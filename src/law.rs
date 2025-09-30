use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Law {
    pub description: String,
}
