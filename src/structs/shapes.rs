use serde::*;

use super::bounds::AMBounds;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AMShapes {
    Basic {
        bounds: Vec<AMBounds>,
        faces: Vec<usize>
    }
}