use bevy::prelude::*;
use bevy_egui::*;
use bevy_panorbit_camera::*;

/**
 * [ ] Asset loader for amb and amj files
 * [ ] File load and change
 * [ ] Modify existing bounds
 * [ ] Create new bounds
 * [ ] Create new faces
 */

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin, EguiPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, ui)
        .run();
}

fn setup(mut commands: Commands) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));
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
