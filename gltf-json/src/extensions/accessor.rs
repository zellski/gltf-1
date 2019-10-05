use std::collections::HashMap;
use serde_json::Value;

use gltf_derive::Validate;
use serde_derive::{Serialize, Deserialize};

/// Contains data structures for sparse storage.
pub mod sparse {
    use super::*;

    /// Indices of those attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Indices {
        #[serde(default, flatten)]
        pub others: HashMap<String, Value>,
    }

    /// Sparse storage of attributes that deviate from their initialization value.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Sparse {
        #[serde(default, flatten)]
        pub others: HashMap<String, Value>,
    }

    /// Array of size `count * number_of_components` storing the displaced
    /// accessor attributes pointed by `accessor::sparse::Indices`.
    #[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
    pub struct Values {
        #[serde(default, flatten)]
        pub others: HashMap<String, Value>,
    }
}

/// A typed view into a buffer view.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Accessor {
    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}
