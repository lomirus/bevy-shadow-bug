use bevy::{core_pipeline::bloom::BloomSettings, input::mouse::MouseMotion, prelude::*};

use crate::{
    pause_menu::PauseMenuUI,
    player::{Player, PLAYER_HEIGHT},
};

const CAMERA_MOTION_SPEED: f32 = 0.0020;
const THIRD_PERSON_VIEW_SIGHT_DISTANCE: f32 = PLAYER_HEIGHT * 2.0;

#[derive(Component)]
pub(crate) struct CameraSettings {
    /// First person view, unless third person view.
    camera_view: CameraView,
}

enum CameraView {
    FirstPerson,
    ThirdPerson,
}

pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(CameraSettings {
        camera_view: CameraView::ThirdPerson,
    });
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.5, 5.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..default()
    });
    commands.spawn(BloomSettings::default());
}

pub(crate) fn update_camera(
    mut camera: Query<&mut Transform, With<Camera3d>>,
    player: Query<&Player>,
    camera_settings: Query<&CameraSettings>,
    game_paused: Query<&PauseMenuUI>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !game_paused.is_empty() {
        return;
    }

    let mut camera = camera.iter_mut().next().unwrap();
    let player = player.iter().next().unwrap();

    for motion in mouse_motion.iter() {
        camera.rotate_y(-CAMERA_MOTION_SPEED * motion.delta.x);

        // Before expand:
        // ```
        // let mut t = camera.clone();
        // t.rotate_local_x(CAMERA_MOTION_UNIT * motion.delta.y);
        // if t.local_y().y > 0.0 {
        //     camera.rotation = t.rotation;
        // }
        // ```
        let new_rotation =
            camera.rotation * Quat::from_rotation_x(-CAMERA_MOTION_SPEED * motion.delta.y);

        // Do nothing if it will turn camera upside down
        if (new_rotation * Vec3::Y).y > 0.0 {
            camera.rotation = new_rotation;
        }
    }

    match camera_settings.iter().next().unwrap().camera_view {
        CameraView::FirstPerson => {
            camera.translation.x = player.coord.x;
            camera.translation.y = player.coord.y + PLAYER_HEIGHT *1.2;
            camera.translation.z = player.coord.z;
        }
        CameraView::ThirdPerson => {
            camera.translation = player.coord + camera.back() * THIRD_PERSON_VIEW_SIGHT_DISTANCE;
            camera.translation.y += PLAYER_HEIGHT / 2.0;
        }
    }
}

pub(crate) fn handle_change_view(
    mut camera_settings: Query<&mut CameraSettings>,
    keyborad_input: Res<Input<KeyCode>>,
) {
    if keyborad_input.just_pressed(KeyCode::V) {
        let mut camera_settings = camera_settings.iter_mut().next().unwrap();
        camera_settings.camera_view = match camera_settings.camera_view {
            CameraView::FirstPerson => CameraView::ThirdPerson,
            CameraView::ThirdPerson => CameraView::FirstPerson,
        }
    }
}
