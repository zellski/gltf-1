use std::collections::HashMap;

use gltf_derive::Validate;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {
    #[cfg(feature = "KHR_lights_punctual")]
    #[serde(default, rename = "KHR_lights_punctual", skip_serializing_if = "Option::is_none")]
    pub khr_lights_punctual: Option<KhrLightsPunctual>,

    #[serde(default, flatten)]
    pub others: HashMap<String, Value>,
}

impl Root {
    pub(crate) fn is_empty(&self) -> bool {
        let empty = self.others.is_empty();

        #[cfg(feature = "KHR_lights_punctual")]
        let empty = empty && self.khr_lights_punctual.is_none();

        return empty;
    }
}

#[cfg(feature = "KHR_lights_punctual")]
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct KhrLightsPunctual {
    /// Lights at this node.
    pub lights: Vec<crate::extensions::scene::khr_lights_punctual::Light>,
}

#[cfg(feature = "KHR_lights_punctual")]
impl crate::root::Get<crate::extensions::scene::khr_lights_punctual::Light> for crate::Root {
    fn get(&self, id: crate::Index<crate::extensions::scene::khr_lights_punctual::Light>)
        -> Option<&crate::extensions::scene::khr_lights_punctual::Light>
    {
        if let Some(khr_lights_punctual) = self.extensions.khr_lights_punctual.as_ref() {
            khr_lights_punctual.lights.get(id.value())
        } else {
            None
        }
    }
}
