use bevy::prelude::*;
use bevy_egui::*;
use egui::*;

// settings resource
#[derive(Debug, Default, Resource, Clone, Copy)]
pub struct AMEditorSettings {
    pub color: Color,
    pub width: f32
}

// plugin for handling settings
pub struct AMEditorSettingsPlugin;
impl Plugin for AMEditorSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AMEditorSettings::default())
            .add_systems(Update, ui);
    }
}

fn ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<AMEditorSettings>,
    mut gizmo_config: ResMut<GizmoConfig>
) {
    // create immovable setting window
    egui::Window::new("Settings")
        .anchor(Align2::RIGHT_TOP, egui::Vec2 { x: 0., y: 0. })
        .movable(false)
        .show(contexts.ctx_mut(), |ui| {
            // add edit color
            let mut color = [settings.color.r(), settings.color.g(), settings.color.b()];
            ui.color_edit_button_rgb(&mut color);
            if color[0] != settings.color.r() || color[1] != settings.color.g() || color[2] != settings.color.b() {
                settings.color = Color::rgb(color[0], color[1], color[2]);
            }

            // line width modifier
            ui.add(egui::Slider::new(&mut gizmo_config.line_width, 0.1 ..=10.0));
        });
}
