use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera).add_system(camera_input_handling);
    }
}

#[derive(Component, Debug, Default)]
struct CameraPivot;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle {
            ..default()
        })
        .insert(CameraPivot)
        .insert(Name::new("CameraPivot"))
        .with_children(|parent| {
            parent.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(5.5, 5.5, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });
}

fn camera_input_handling(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraPivot)>
) {
    let mut localTranslation = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        localTranslation.z -= 0.01;
    }

    if keys.pressed(KeyCode::S) {
        localTranslation.z += 0.01;
    }

    if keys.pressed(KeyCode::A) {
        localTranslation.x -= 0.01;
    }

    if keys.pressed(KeyCode::D) {
        localTranslation.x += 0.01;
    }

    for (mut transform, _) in query.iter_mut() {
        transform.translation += localTranslation;
    }
}