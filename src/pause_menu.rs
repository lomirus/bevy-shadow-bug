use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Component)]
pub(crate) struct PauseMenuUI;

pub(crate) fn handle_pause_menu(
    mut commands: Commands,
    pause_menu: Query<Entity, With<PauseMenuUI>>,
    keyborad_input: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyborad_input.just_pressed(KeyCode::Escape) {
        let mut window = window.get_single_mut().unwrap();
        if pause_menu.is_empty() {
            commands.spawn((
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                PauseMenuUI,
            ));

            window.cursor_options.visible = true;
            window.cursor_options.grab_mode = CursorGrabMode::None;
        } else {
            for pause_menu in pause_menu.iter() {
                commands.entity(pause_menu).despawn();
            }

            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}
