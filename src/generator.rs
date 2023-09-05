use bevy::prelude::*;

use crate::structs::{shapes::AMShape, bounds::AMBounds};

#[derive(Default, Debug, Clone, Copy)]
pub struct BoundInfo {
    pub pos_offset: usize,
    pub pos_length: usize
}

#[derive(Default, Debug, Clone)]
pub struct ShapeInfo {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>
}

// function to generate mesh info from a shape
pub fn gen_shape_mesh(shape: AMShape) -> ShapeInfo {
    // setup
    let mut infos = vec![BoundInfo::default(); shape.bounds.len()];

    // generate positions
    let mut positions: Vec<Vec3> = Vec::new();
    shape.bounds.iter().enumerate().for_each(|(index, bound)| {
        gen_positions(bound, &mut positions, infos.get_mut(index).unwrap());
    });

    // generate faces (uvs and indices)
    let mut indices = Vec::new();
    for i in (0 .. shape.faces.len()).step_by(2) {
        gen_indices(
            infos.get(i).unwrap(), 
            infos.get(i + 1).unwrap(), 
            &mut indices
        );
    }
    
    return ShapeInfo { positions, indices };
}

// function to generate positions of a mesh and update relevant infos
pub fn gen_positions(bound: &AMBounds, positions: &mut Vec<Vec3>, info: &mut BoundInfo) {
    // set position offset
    info.pos_offset = positions.len();

    // generate points
    match bound {
        AMBounds::Point { position } => {
            positions.push(position.clone());
            info.pos_length = 1;
        },
        AMBounds::Curve { start_point, end_point, start_direction, end_direction } =>{
            // get number of points to generate (5 * distance convered)
            let num_points = start_point.distance(*end_point).round() * 5.;
            info.pos_length = num_points as usize + 1;

            // add points to vec
            for n in 0 .. info.pos_length {
                // get percent
                let percent = n as f32 / num_points;
                let opposite_percent = 1.0 - percent;

                // create multipliers
                let p0multiplier = opposite_percent * opposite_percent * opposite_percent;
                let p1multiplier = percent * percent * percent;
                let c0multiplier = 3.0 * (opposite_percent * opposite_percent) * percent;
                let c1multiplier = 3.0 * opposite_percent * (percent * percent);

                // push point
                positions.push(Vec3::new(
                    (p0multiplier * start_point.x) + (c0multiplier * start_direction.x) + (c1multiplier * end_direction.x) + (p1multiplier * end_point.x),
                    (p0multiplier * start_point.y) + (c0multiplier * start_direction.y) + (c1multiplier * end_direction.y) + (p1multiplier * end_point.y),
                    (p0multiplier * start_point.z) + (c0multiplier * start_direction.z) + (c1multiplier * end_direction.z) + (p1multiplier * end_point.z)
                ));
            }
        },
    }
}

pub fn gen_indices(a: &BoundInfo, b: &BoundInfo, indices: &mut Vec<u32>) {
    // get which points array is smallest
    let is_a_smallest = a.pos_length < b.pos_length;
    let largest_length = if is_a_smallest { b.pos_length } else { a.pos_length };
    let smallest_length = if is_a_smallest { a.pos_length } else { b.pos_length };
    let largest_offset = if is_a_smallest { b.pos_offset } else { a.pos_offset };
    let smallest_offset = if is_a_smallest { a.pos_offset } else { b.pos_offset };

    // divide the size of the largest by the size of the smallest to determine how often the smaller index needs to increment when looping through the larger array
    let small_increment_interval = largest_length / smallest_length;
    let mut small_index = 0;

    // loop through each point on the largest, result in a pair (index 0 and 1, or index 3 and 4)
    let loop_size = largest_length - 1;
    for n in 0..loop_size {
        // add indexes of the point pair from the largest points array to the output
        indices.push((largest_offset + n) as u32);
        indices.push((largest_offset + n + 1) as u32);

        // add index of the point in the smallest points array to the output
        indices.push((smallest_offset + small_index) as u32);

        // if the small index needs to increment
        if n % small_increment_interval == 0 && n != loop_size && n != 0 {
            // increment
            small_index += 1;

            // add second point from above point pair from the largest points array to the output
            indices.push((largest_offset + n + 1) as u32);

            // add the old and new index of the points from the small est points array to the output
            indices.push((smallest_offset + small_index) as u32);
            indices.push((smallest_offset + small_index - 1) as u32);
        }
    }
}
