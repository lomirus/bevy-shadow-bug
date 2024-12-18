use bevy::prelude::*;

use crate::pause_menu::PauseMenuUI;

pub(crate) const PLAYER_HEIGHT: f32 = 64.0;
const PLAYER_MOVE_SPEED: f32 = 0.1;

#[derive(Component, Reflect)]
pub(crate) struct Player {
    pub(crate) coord: Vec3,
}

pub(crate) fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {
            coord: Vec3::new(0.0, 0.0, 0.0),
        },
        SceneRoot(asset_server.load("player.glb#Scene0")),
    ));
}

pub(crate) fn handle_player_move(
    mut player: Query<&mut Player>,
    game_paused: Query<&PauseMenuUI>,
    camera: Query<&Transform, With<Camera3d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !game_paused.is_empty() {
        return;
    }

    let mut player = player.iter_mut().next().unwrap();
    let camera = camera.iter().next().unwrap();

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::KeyS) {
        let forward = camera.forward();
        let forward = Vec2::new(forward.x, forward.z).normalize();

        if keyboard_input.pressed(KeyCode::KeyW) {
            player.coord.x += forward.x * PLAYER_MOVE_SPEED;
            player.coord.z += forward.y * PLAYER_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            player.coord.x -= forward.x * PLAYER_MOVE_SPEED;
            player.coord.z -= forward.y * PLAYER_MOVE_SPEED;
        }
    }

    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::KeyD) {
        let left = camera.left();

        if keyboard_input.pressed(KeyCode::KeyA) {
            player.coord.x += left.x * PLAYER_MOVE_SPEED;
            player.coord.z += left.z * PLAYER_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.coord.x -= left.x * PLAYER_MOVE_SPEED;
            player.coord.z -= left.z * PLAYER_MOVE_SPEED;
        }
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        player.coord.y -= PLAYER_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        player.coord.y += PLAYER_MOVE_SPEED;
    }
}

pub(crate) fn update_player_model(mut player: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = player.iter_mut().next().unwrap();
    transform.translation = Vec3::new(player.coord.x, player.coord.y, player.coord.z);
}
