use bevy::{prelude::*, ecs::system::*};
use bevy_rapier3d::prelude::Collider;

use crate::AMEditorContext;

// selector collider root component
#[derive(Component)]
pub struct SelectorColliderRoot;

// selector collider component
#[derive(Component)]
pub struct SelectorCollider {
    pub shape_idx:  usize,
    pub bounds_idx: usize,
    pub point_idx:  usize
}

// create selector plugin
pub struct AMEditorSelectorPlugin;
impl Plugin for AMEditorSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_selection_colliders);
    }
}

fn update_selection_colliders(
    mut commands: Commands,
    query: Query<(Entity, &AMEditorContext, &Children), Changed<AMEditorContext>>,
    collider_root: Query<(Entity, &SelectorColliderRoot)>
) {
    query.for_each(|(parent, context, parent_children)| {
        // remove old root
        parent_children.iter().for_each(|e| {
            // if collider_root.contains(*e) {
                commands.entity(*e).despawn_recursive();
            // }
        });

        match &context.mode {
            crate::modes::AMEditorMode::SelectGroup { positions } => {
                // todo seperate colliders for each bound with a marker component for proper selection handling
                commands.entity(parent).with_children(|builder| {
                    let mut new_entity = builder.spawn((SelectorColliderRoot, VisibilityBundle::default(), TransformBundle::default()));
                    install_colliders_to_entity(&mut new_entity, positions);
                });
            },

            crate::modes::AMEditorMode::None => todo!(),
            crate::modes::AMEditorMode::SelectBound { index: _ } => todo!(),
            crate::modes::AMEditorMode::SelectPoint { bound_index: _, name: _, value: _ } => todo!(),
        }
    });
}

fn install_colliders_to_entity(
    builder: &mut EntityCommands,
    positions: &Vec<Vec3>
) {
    let mut colliders = Vec::with_capacity(positions.len() - 1);

    // add a child to the builder for each pair of positions
    for i in 0 .. positions.len() - 1 {
        // get first and second point, as well as, the mid point
        let a = positions[i];
        let b = positions[i + 1];
        let length = a.distance(b);
        let mid = (b - a) / 2.0 + a;

        // create transform to perform look at (bevy please)
        let mut transform = Transform { translation: a, ..Default::default() };
        transform.look_at(b, Vec3::X);
        transform.rotate_axis(transform.left(), 90.0_f32.to_radians());

        // add the entity
        colliders.push((
            mid, transform.rotation,
            Collider::cylinder(length / 2.0, 0.1)
        ));
    }

    builder.insert(Collider::compound(colliders));
}
