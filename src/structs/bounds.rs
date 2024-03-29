use bevy_math::Vec3;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum AMBounds {
    Point {
        #[serde(with = "super::vec3_array")]
        position: Vec3
    },
    Linear {
        #[serde(with = "super::vec3_array")]
        start_point: Vec3,
        #[serde(with = "super::vec3_array")]
        end_point: Vec3
    },
    Curve {
        #[serde(with = "super::vec3_array")]
        start_point: Vec3,
        #[serde(with = "super::vec3_array")]
        end_point: Vec3,
        #[serde(with = "super::vec3_array")]
        start_direction: Vec3,
        #[serde(with = "super::vec3_array")]
        end_direction: Vec3,
    }
}