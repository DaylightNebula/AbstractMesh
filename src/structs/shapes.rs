use serde::*;

use super::bounds::AMBounds;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct AMShape {
    pub bounds: Vec<AMBounds>,
    pub faces: Vec<usize>
}