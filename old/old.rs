/*
use std::ops::Add;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::ser::serialize_tagged_newtype;
use bevy::render::mesh;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_resource::ShaderType;
use bevy_prototype_debug_lines::*;

// todo proper normals calculation
// todo face function to generate from two slices
// todo generate via json
// todo allow for point generation type
// todo allow for circle generation type

struct Curve {
    point0: Vec3,
    point1: Vec3,
    control0: Vec3,
    control1: Vec3
}

const TEST_CURVE: Curve = Curve {
    point0: Vec3 { x: 1.0, y: 0.0, z: 0.0 },
    point1: Vec3 { x: -1.0, y: 0.0, z: 0.0 },
    control0: Vec3 { x: 1.0, y: 0.0, z: 1.0 },
    control1: Vec3 { x: -1.0, y: 0.0, z: -1.0 },
};
const NUM_POINTS_CURVE: i32 = 31;

const TEST_LINE: Curve = Curve {
    point0: Vec3 { x: 1.0, y: -1.0, z: 0.0 },
    point1: Vec3 { x: -1.0, y: -1.0, z: 0.0 },
    control0: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
    control1: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
};
const NUM_POINT_LINE: i32 = 1;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_system(update)
        .add_startup_system(start)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // convert curves to point lists
    let mut curve_array = gen_points_from_curve(&TEST_CURVE, &NUM_POINTS_CURVE);
    let mut line_array = gen_points_from_curve(&TEST_LINE, &NUM_POINT_LINE);

    // generate the indices of the mesh
    let indices = gen_indices(&curve_array, &line_array);

    // create mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // combine and set vertices
    let mut positions = Vec::new();
    positions.append(&mut curve_array);
    positions.append(&mut line_array);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    // set normals
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![Vec3::new(0.0, 0.0, -1.0); (NUM_POINT_LINE + NUM_POINTS_CURVE + 2) as usize]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, gen_uvs(&NUM_POINTS_CURVE, &NUM_POINT_LINE));

    // set indices
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    // spawn mesh
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
}


fn update(mut lines: ResMut<DebugLines>) {
    // // draw cross at each point
    draw_cross(&mut lines, &TEST_CURVE.point0, 0.25);
    draw_cross(&mut lines, &TEST_CURVE.point1, 0.25);

    // draw the test curve
    // let curve_array = gen_points_from_curve(&TEST_CURVE, &NUM_POINTS_CURVE);
    // draw_line(&mut lines, &curve_array);
    //
    // // draw the test line
    // let line_array = gen_points_from_curve(&TEST_LINE, &NUM_POINT_LINE);
    // draw_line(&mut lines, &line_array);
    //
    // // gen mesh
    // let indices = gen_indices(&curve_array, &line_array);
    // draw_indices(&mut lines, &curve_array, &line_array, &indices);
}

// generate normals for all the positions
fn gen_normals(curveA: &Curve, curveB: &Curve) -> Vec<Vec3> {
    let mut output = Vec::new();
    return output;
}

// generate vector of uvs, the pts 1 array will go along the top, and pts2 array will go along the bottom of the texture
fn gen_uvs(pts1_len: &i32, pts2_len: &i32) -> Vec<Vec2> {
    let mut output = Vec::new();

    for n in 0 .. (*pts1_len + 1) {
        output.push(Vec2::new(n as f32 / *pts1_len as f32, 0.0));
    }

    for n in 0 .. (*pts2_len + 1) {
        output.push(Vec2::new(n as f32 / *pts2_len as f32, 1.0));
    }

    return output;
}

// Using the two points arrays, it generates a list of indices that connects both.
// Indices in the second array are offset by the size of the first array to differentiate the two.
// Length of the larger array must be divisible by the size of the smaller array (easy if lengths are 2 ^ n, ex: 2, 4, 8, 16, 32, etc)
fn gen_indices(pts1: &Vec<Vec3>, pts2: &Vec<Vec3>) -> Vec<u32> {
    // get which points array is smallest
    let is_pts1_smallest = pts1.size() < pts2.size();
    let smallest = if is_pts1_smallest { pts1 }  else { pts2 };
    let largest = if is_pts1_smallest { pts2 } else { pts1 };
    let largest_offset = if is_pts1_smallest { pts1.len() as i32 } else { 0 };
    let smallest_offset = if !is_pts1_smallest { pts1.len() as i32 } else { 0 };

    // divide the size of the largest by the size of the smallest to determine how often the smaller index needs to increment when looping through the larger array
    let small_increment_interval = largest.len() as i32 / smallest.len() as i32;
    let mut small_index = 0;

    // setup output
    let mut output: Vec<u32> = Vec::new();

    // loop through each point on the largest, result in a pair (index 0 and 1, or index 3 and 4)
    let loop_size = largest.len() as i32 - 1;
    for n in 0..loop_size {
        // add indexes of the point pair from the largest points array to the output
        output.push((largest_offset + n) as u32);
        output.push((largest_offset + n + 1) as u32);

        // add index of the point in the smallest points array to the output
        output.push((smallest_offset + small_index) as u32);

        // if the small index needs to increment
        if n % small_increment_interval == 0 && n != loop_size && n != 0 {
            // increment
            small_index += 1;

            // add second point from above point pair from the largest points array to the output
            output.push((largest_offset + n + 1) as u32);

            // add the old and new index of the points from the small est points array to the output
            output.push((smallest_offset + small_index) as u32);
            output.push((smallest_offset + small_index - 1) as u32);
        }
    }

    return output;
}

// draws a wireframe from the indices provided
fn draw_indices(lines: &mut ResMut<DebugLines>, pts1: &Vec<Vec3>, pts2: &Vec<Vec3>, indices: &Vec<u32>) {
    for n in 0..(indices.len() as i32 / 3) {
        let a = eval_index(pts1, pts2, indices[n as usize * 3] as i32);
        let b = eval_index(pts1, pts2, indices[n as usize * 3 + 1] as i32);
        let c = eval_index(pts1, pts2, indices[n as usize * 3 + 2] as i32);
        lines.line(a, b, 0.0);
        lines.line(b, c, 0.0);
        lines.line(a, c, 0.0);
    }
}

// takes a given index, if it is greater than the length of the first array, it try's to get the value from the second array.
fn eval_index(pts1: &Vec<Vec3>, pts2: &Vec<Vec3>, index: i32) -> Vec3 {
    return if index >= pts1.len() as i32 { pts2[index as usize - pts1.len()] } else { pts1[index as usize] }
}

// takes in a curve and generates points along it
fn gen_points_from_curve(curve: &Curve, num_points: &i32) -> Vec<Vec3> {
    let mut array = Vec::new();
    for n in 0 .. (num_points + 1) {
        let i: f32 = n as f32 / *num_points as f32;
        array.push(get_curve(curve, i));
    }
    return array;
}

fn draw_line(lines: &mut ResMut<DebugLines>, array: &Vec<Vec3>) {
    for i in 0..(array.len() as i32 - 1) {
        lines.line(array[i as usize], array[(i + 1) as usize], 0.0)
    }
}

// draw a cross at the given position with the given size
fn draw_cross(lines: &mut ResMut<DebugLines>, vec: &Vec3, size: f32) {
    let right = vec.clone().add(Vec3::new(size, 0.0, 0.0));
    let left = vec.clone().add(Vec3::new(-size, 0.0, 0.0));
    let up = vec.clone().add(Vec3::new(0.0, size, 0.0));
    let down = vec.clone().add(Vec3::new(0.0, -size, 0.0));
    let forward = vec.clone().add(Vec3::new(0.0, 0.0, size));
    let backward = vec.clone().add(Vec3::new(0.0, 0.0, -size));
    lines.line_gradient(right, left, 0.0, Color::GREEN, Color::BLUE);
    lines.line_gradient(forward, backward, 0.0, Color::GOLD, Color::PINK);
    lines.line_gradient(up, down, 0.0, Color::PINK, Color::BLUE);
}

// get vec3 from curve and time
fn get_curve(curve: &Curve, percent: f32) -> Vec3 {
    let opposite_percent = 1.0 - percent;
    let p0multiplier = opposite_percent * opposite_percent * opposite_percent;
    let p1multiplier = percent * percent * percent;
    let c0multiplier = 3.0 * (opposite_percent * opposite_percent) * percent;
    let c1multiplier = 3.0 * opposite_percent * (percent * percent);
    return Vec3::new(
        (p0multiplier * curve.point0.x) + (c0multiplier * curve.control0.x) + (c1multiplier * curve.control1.x) + (p1multiplier * curve.point1.x),
        (p0multiplier * curve.point0.y) + (c0multiplier * curve.control0.y) + (c1multiplier * curve.control1.y) + (p1multiplier * curve.point1.y),
        (p0multiplier * curve.point0.z) + (c0multiplier * curve.control0.z) + (c1multiplier * curve.control1.z) + (p1multiplier * curve.point1.z)
    );
}*/