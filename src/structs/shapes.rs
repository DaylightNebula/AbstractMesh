use std::{fs::File, io::BufReader};

use serde::*;

use super::bounds::AMBounds;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub struct AMShape {
    pub bounds: Vec<AMBounds>,
    pub faces: Vec<isize>,

    #[serde(default)]
    pub inversion_list: Vec<bool>
}

#[derive(Debug)]
pub enum ShapeLoadError {
    FileLoad(String),
    Deserialize(String)
}

pub fn load_shapes(path: impl Into<String>) -> Result<Vec<AMShape>, ShapeLoadError> {
    // load content
    let content = std::fs::read_to_string(path.into());
    let content = 
        if content.is_ok() { content.unwrap() } 
        else { return Err(ShapeLoadError::FileLoad(format!("{:?}", content.err().unwrap()))) };

    // load shapes
    let shapes = serde_json::from_str::<Vec<AMShape>>(content.as_str());
    let mut shapes =
        if shapes.is_ok() { shapes.unwrap() }
        else { return Err(ShapeLoadError::Deserialize(format!("{:?}", shapes.err().unwrap()))) };

    // load inversion map
    shapes.iter_mut().for_each(|shape| {
        shape.inversion_list = Vec::with_capacity(shape.faces.len());
        
        // for all face elements, make sure they are positive
        // if any are negative, make them positive and save the inversion
        (0 .. shape.faces.len()).for_each(|index| {
            if shape.faces[index] < 0 {
                shape.faces[index] *= -1;
                shape.inversion_list.push(true);
            } else {
                shape.inversion_list.push(false); // todo test without
            }
            shape.faces[index] -= 1;
        });
    });

    // return final shapes
    return Ok(shapes);
}

pub fn load_shapes_bin(path: impl Into<String>) -> Result<Vec<AMShape>, ShapeLoadError> {
    
    // load content
    // let content = std::fs::read(path.into());
    // let content = 
    //     if content.is_ok() { content.unwrap() } 
    //     else { return Err(ShapeLoadError::FileLoad(format!("{:?}", content.err().unwrap()))) };
    let file = File::open(path.into()).unwrap();
    let mut shapes: Vec<AMShape> = serde_cbor::from_reader(file).unwrap();

    // load shapes
    // let shapes = serde_cbor::from_str::<Vec<AMShape>>(content.as_str());
    // let shapes = serde_cbor::from_reader::<Vec<AMShape>>(BufReader::new(File::open(path.into()).unwrap()));
    // let mut shapes =
    //     if shapes.is_ok() { shapes.unwrap() }
    //     else { return Err(ShapeLoadError::Deserialize(format!("{:?}", shapes.err().unwrap()))) };

    // load inversion map
    // shapes.iter_mut().for_each(|shape| {
    //     shape.inversion_list = Vec::with_capacity(shape.faces.len());
        
    //     // for all face elements, make sure they are positive
    //     // if any are negative, make them positive and save the inversion
    //     (0 .. shape.faces.len()).for_each(|index| {
    //         if shape.faces[index] < 0 {
    //             shape.faces[index] *= -1;
    //             shape.inversion_list.push(true);
    //         } else {
    //             shape.inversion_list.push(false); // todo test without
    //         }
    //         shape.faces[index] -= 1;
    //     });
    // });

    // return final shapes
    return Ok(shapes);
}

pub fn save_bin(path: impl Into<String>, shapes: Vec<AMShape>) {
    let file = File::create(path.into()).unwrap();
    let _ = serde_cbor::to_writer(file, &shapes);
}
