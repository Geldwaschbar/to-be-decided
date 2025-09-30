use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Resource {
    #[default]
    Money,
    Popularity,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ModType {
    // v = n
    Setter,
    // v += n
    #[default]
    Constant,
    // v *= n
    Multiplier,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Modifier {
    pub mod_type: ModType,
    pub resource: Resource,
    pub value: f64,
}
