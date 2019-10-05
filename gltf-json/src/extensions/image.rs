use std::collections::HashMap;
use serde_json::Value;

use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// Image data used to create a texture.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Image {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
