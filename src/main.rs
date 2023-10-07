use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use components::{Movable, Velocity};

mod components;
mod player;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My bevy game".to_string(),
                    ..default()
                }),
                ..default()
            }),
            player::PlayerPlugin,
        ))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup_system)
        .add_systems(Update, movable_system)
        .run();
}

fn setup_system(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle::default());

    let Ok(primary) = query.get_single() else {
        return;
    };

    let win_size = WinSize {
        w: primary.width(),
        h: primary.height(),
    };
    commands.insert_resource(win_size);
}

fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Movable>>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * BASE_SPEED * TIME_STEP;
        translation.y += velocity.y * BASE_SPEED * TIME_STEP;

        if movable.auto_despawn {
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h + MARGIN
                || translation.y < -win_size.h - MARGIN
                || translation.x > win_size.w + MARGIN
                || translation.x < -win_size.w - MARGIN
            {
                println!("->> despawn {entity:?}");
                commands.entity(entity).despawn();
            }
        }
    }
}
