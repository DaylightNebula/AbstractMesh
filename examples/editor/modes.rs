use bevy::prelude::*;

use crate::{AMEditorContext, settings::AMEditorSettings};

// plugin to run any handling functions for the editor mode
pub struct AMEditorModePlugin;
impl Plugin for AMEditorModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_mode);
    }
}

// the actual mode enum
#[derive(Debug, Default, Clone)]
pub enum AMEditorMode {
    #[default]
    None,
    SelectGroup { positions: Vec<Vec3> },
    SelectBound { index: usize },
    SelectPoint { bound_index: usize, name: String, value: Vec3 }
}

// function for handling current mode
pub fn handle_mode(
    query: Query<&AMEditorContext>,
    settings: Res<AMEditorSettings>,
    mut gizmos: Gizmos
) {
    // for each in query, and match to handling function
    query.for_each(|context| {
        match &context.mode {
            AMEditorMode::None => {}

            // draw group with gizmos
            AMEditorMode::SelectGroup { positions } => {
                for i in 0 .. positions.len() - 1 {
                    gizmos.line(positions[i], positions[i + 1], settings.color);
                }
            },
            AMEditorMode::SelectBound { index: _ } => todo!(),
            AMEditorMode::SelectPoint { bound_index: _, name: _, value: _ } => todo!(),
        }
    });
}