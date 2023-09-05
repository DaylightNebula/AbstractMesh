use bevy::{prelude::*, render::mesh::Indices};
use generator::gen_shape_mesh;
use structs::shapes::AMShape;

pub mod generator;
pub mod structs;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // generate test mesh
    let file = std::fs::read_to_string("./assets/test.amj");
    let shapes = serde_json::from_str::<Vec<AMShape>>(file.unwrap().as_str()).unwrap();
    for shape in shapes {
        let info = gen_shape_mesh(shape); // todo combine all shapes into one mesh
        let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, info.positions);
        mesh.set_indices(Some(Indices::U32(info.indices)));

        // spawn mesh
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        });
    }
}

fn update(mut gizmos: Gizmos) {
    // load file
    let file = std::fs::read_to_string("./assets/test.amj");
    let shapes = serde_json::from_str::<Vec<AMShape>>(file.unwrap().as_str()).unwrap();
    
    // draw shapes via gizmos
    for shape in shapes {
        // generate shape
        let info = gen_shape_mesh(shape);
        let indices = info.indices;

        // draw indices
        for n in (0 .. indices.len()).step_by(3) {
            let a = info.positions.get(*indices.get(n).unwrap() as usize).unwrap();
            let b = info.positions.get(*indices.get(n + 1).unwrap() as usize).unwrap();
            let c = info.positions.get(*indices.get(n + 2).unwrap() as usize).unwrap();
            gizmos.line(*a, *b, Color::RED);
            gizmos.line(*a, *c, Color::BLUE);
            gizmos.line(*c, *b, Color::GREEN);
        }
    }
}
