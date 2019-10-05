use std::collections::HashMap;
use serde_json::Value;

use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Skin {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
