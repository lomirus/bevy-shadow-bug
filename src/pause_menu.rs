use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};

#[derive(Component)]
pub(crate) struct PauseMenuUI;

pub(crate) fn handle_pause_menu(
    mut commands: Commands,
    pause_menu: Query<Entity, With<PauseMenuUI>>,
    keyborad_input: Res<Input<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyborad_input.just_pressed(KeyCode::Escape) {
        let mut window = window.get_single_mut().unwrap();
        if pause_menu.is_empty() {
            commands.spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                },
                PauseMenuUI,
            ));

            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        } else {
            for pause_menu in pause_menu.iter() {
                commands.entity(pause_menu).despawn();
            }

            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
    }
}
