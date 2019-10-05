use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mesh {}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Primitive {
    #[serde(default, flatten, skip_serializing_if = "HashMap::is_empty")]
    pub others: HashMap<String, Value>,
}

impl Primitive {
    pub(crate) fn is_empty(&self) -> bool {
        self.others.is_empty()
    }
}
