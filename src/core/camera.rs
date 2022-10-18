use bevy::{ prelude::*, input::mouse::MouseWheel };

use std::{ f32::consts::PI };

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera).add_system(camera_input_handling);
    }
}

#[derive(Component, Debug, Default)]
struct CameraPivot;

#[derive(Component, Debug, Default)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 4.0, 0.0)),
            ..default()
        })
        .insert(CameraPivot)
        .insert(Name::new("CameraPivot"))
        .with_children(|parent| {
            parent
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                })
                .insert(MainCamera);
        });
}

fn camera_input_handling(
    keys: Res<Input<KeyCode>>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut pivot: Query<(&mut Transform, &CameraPivot)>,
    mut camera: Query<(&mut Transform, &MainCamera), Without<CameraPivot>>
) {
    let (mut transform, _) = pivot.iter_mut().last().unwrap();
    let pivot_transform = transform.clone();
    let mut translation = Vec3::ZERO;
    let mut scroll: f32 = 0.0;

    if keys.pressed(KeyCode::P) {
        dbg!(transform.rotation);
        dbg!(transform.rotation.to_euler(EulerRot::XZY));
    }

    if keys.pressed(KeyCode::W) {
        translation -= pivot_transform.rotation * Vec3::Z;
    }

    if keys.pressed(KeyCode::S) {
        translation += pivot_transform.rotation * Vec3::Z;
    }

    if keys.pressed(KeyCode::D) {
        translation += pivot_transform.rotation * Vec3::X;
    }

    if keys.pressed(KeyCode::A) {
        translation -= pivot_transform.rotation * Vec3::X;
    }

    if let Some(vec) = translation.try_normalize() {
        transform.translation += vec * 0.01;
    }

    if keys.pressed(KeyCode::Q) {
        transform.rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 0.01, 0.0));
    }

    if keys.pressed(KeyCode::E) {
        transform.rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, -0.01, 0.0));
    }

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    let (mut camera_transform, _) = camera.iter_mut().last().unwrap();
    if scroll.abs() > 0.0 {
        camera_transform.translation -= (scroll * Vec3::Y + scroll * Vec3::Z) * 0.1;
    }
}