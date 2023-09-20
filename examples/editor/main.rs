use abstracted_mesh_rust::{structs::shapes::AMShape, generator::{gen_shape_mesh, BoundInfo, self}};
use bevy::{prelude::*, render::mesh::Indices};
use bevy_egui::*;
use bevy_panorbit_camera::*;
use modes::{AMEditorMode, AMEditorModePlugin};
use settings::AMEditorSettingsPlugin;

/** "Long" term todo list
 * [x] Asset loader for amb and amj files
 * [x] File load
 * [ ] Save file on modle change
 * [ ] Modify existing bounds
 * [ ] Create new bounds
 * [ ] Create new faces
 */

/** "Short" term todo list
 * [ ] Allow bounds to be selected
 * - [ ] Move points around
 * - [ ] Move control points around
 * - [ ] Allow type change (auto generated points if more are needed)
 */

mod loader;
mod modes;
mod settings;

#[derive(Debug, Component, Clone)]
pub struct AMEditorFile {
    pub path: String
}

#[derive(Debug, Default, Component, Clone)]
pub struct AMEditorContext {
    pub shapes: Vec<AMShape>,
    pub mode: AMEditorMode
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin, EguiPlugin))
        .add_plugins((AMEditorModePlugin, AMEditorSettingsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (ui, update_root_object))
        .run();
}

fn setup(mut commands: Commands) {
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
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         color: Color::WHITE,
    //         illuminance: 10000.0,
    //         ..Default::default()
    //     },
    //     transform: Transform::from_xyz(100.0, 200.0, 100.0),
    //     ..Default::default()
    // });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));

    // spawn root object
    commands.spawn((
        AMEditorFile { path: format!("./assets/test.amj") },
        AMEditorContext::default(),
        VisibilityBundle::default(),
        TransformBundle::default()
    ));
}

fn ui(mut contexts: EguiContexts, mut dnd_events: EventReader<FileDragAndDrop>) {
    dnd_events.iter().for_each(|event| {
        match event {
            FileDragAndDrop::DroppedFile { window: _, path_buf } => println!("Drop file {}", path_buf.display()),
            FileDragAndDrop::HoveredFile { window: _, path_buf: _ } => {},
            FileDragAndDrop::HoveredFileCanceled { window: _ } => {},
        }
    });

    egui::Window::new("Abstract Mesh").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open...").clicked() {

            }

            if ui.button("Save...").clicked() {

            }
        });
    });
}

// system that updates the root entity with its file changes
fn update_root_object(
    mut commands: Commands,
    mut query: Query<(Entity, &AMEditorFile, &mut AMEditorContext), Changed<AMEditorFile>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // for each entity
    query.for_each_mut(|(entity, file, mut context)| {
        // load shapes based on extension
        let ext = file.path.split(".").last().unwrap();
        let shapes = match ext {
            "amb" => {
                let shapes = abstracted_mesh_rust::structs::shapes::read_shapes_from_bin_file(file.path.clone());
                if shapes.is_ok() {
                    shapes.unwrap()
                } else {
                    error!("Unable to load shapes with error: {:?}", shapes.err().unwrap());
                    return
                }
            }
            "amj" => {
                let shapes = abstracted_mesh_rust::structs::shapes::read_shapes_from_file(file.path.clone());
                if shapes.is_ok() {
                    shapes.unwrap()
                } else {
                    error!("Unable to load shapes with error: {:?}", shapes.err().unwrap());
                    return
                }
            }
            _ => {
                error!("Unknown file extension {}", ext);
                return
            }
        };

        // save shapes
        context.shapes = shapes;

        // update current selection
        let mut positions = Vec::new();
        let mut info = BoundInfo::default();
        for shape in &context.shapes {
            for bound in &shape.bounds {
                generator::gen_positions(bound, &mut positions, &mut info);
            }
        }
        context.mode = AMEditorMode::SelectGroup { positions };

        // remove all old children
        let mut entity_commands = commands.entity(entity);
        entity_commands.despawn_descendants();

        // spawn a new child for each shape in shapes
        entity_commands.with_children(|builder| {
            for shape in (&context.shapes).into_iter() {
                // generate shape info and unpack
                let info = gen_shape_mesh(shape);
                let positions = info.positions;
                let normals = info.normals;
                
                // generate mesh and update values
                let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.set_indices(Some(Indices::U32(info.indices)));
        
                // temp normals
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        
                // spawn mesh
                builder.spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    ..default()
                });
            }
        });
    });
}
