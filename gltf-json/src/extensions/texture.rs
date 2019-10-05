use std::collections::HashMap;
use serde_json::Value;

use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Sampler {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}

/// A texture and its sampler.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Texture {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
/// Reference to a `Texture`.
pub struct Info {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
