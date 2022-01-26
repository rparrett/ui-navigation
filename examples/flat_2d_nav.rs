use bevy::prelude::*;

use bevy_ui_navigation::systems::{
    default_gamepad_input, default_keyboard_input, default_mouse_input, InputMapping,
};
use bevy_ui_navigation::{Focusable, NavEvent, NavigationPlugin};

/// This example illustrates how to mark buttons as focusable and let
/// NavigationPlugin figure out how to go from one to another.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 1: Add the NavigationPlugin
        .add_plugin(NavigationPlugin)
        .init_resource::<InputMapping>()
        .add_startup_system(setup)
        .add_system(button_system)
        .add_system(default_keyboard_input)
        .add_system(default_gamepad_input)
        .add_system(default_mouse_input)
        .add_system(print_nav_events)
        .run();
}

fn button_system(mut interaction_query: Query<(&Focusable, &mut UiColor), Changed<Focusable>>) {
    for (focus_state, mut material) in interaction_query.iter_mut() {
        if focus_state.is_focused() {
            *material = Color::ORANGE_RED.into();
        } else {
            *material = Color::DARK_GRAY.into();
        }
    }
}
fn print_nav_events(mut events: EventReader<NavEvent>) {
    for event in events.iter() {
        println!("{:?}", event);
    }
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let positions = [
        Vec2::new(10.0, 10.0),
        Vec2::new(15.0, 50.0),
        Vec2::new(20.0, 90.0),
        Vec2::new(30.0, 10.0),
        Vec2::new(35.0, 50.0),
        Vec2::new(40.0, 90.0),
        Vec2::new(60.0, 10.0),
        Vec2::new(55.0, 50.0),
        Vec2::new(50.0, 90.0),
    ];
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|commands| {
            for pos in positions {
                spawn_button(pos, commands);
            }
        });
}
fn spawn_button(position: Vec2, commands: &mut ChildBuilder) {
    let position = Rect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..Default::default()
    };
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(95.0), Val::Px(65.0)),
                position,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::DARK_GRAY.into(),
            ..Default::default()
        })
        // 2. Add the `Focusable` component to the navigable Entity
        .insert(Focusable::default());
}
