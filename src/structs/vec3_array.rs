use bevy_math::*;
use serde::*;

// array intermeidiate for vec3's
#[derive(Serialize, Deserialize)]
pub struct Vec3Inter(pub [f32; 3]);

// serialize vec3 -> inter -> result
pub fn serialize<S>(input: &Vec3, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let inter = Vec3Inter([ input.x, input.y, input.z ]);
    inter.serialize(serializer)
}

// deserialize input -> inter -> vec3
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec3, D::Error> where D: Deserializer<'de> {
    let inter = Vec3Inter::deserialize(deserializer);
    if inter.is_err() { return Err(inter.err().unwrap()) }
    let inter = inter.unwrap();
    Ok(Vec3 { x: inter.0[0], y: inter.0[1], z: inter.0[2] })
}