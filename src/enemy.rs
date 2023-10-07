use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, SpriteSize},
    WinSize,
};

const ENEMY_COLOR: Color = Color::GRAY;
const ENEMY_SIZE: Vec2 = Vec2::new(40., 40.);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_enemy_system);
    }
}

fn spawn_enemy_system(mut commands: Commands, win_size: Res<WinSize>) {
    let mut rng = thread_rng();
    let x_span = win_size.w / 2. - 100.;
    let y_span = win_size.h / 2. - 100.;

    let x = rng.gen_range(-x_span..x_span);
    let y = rng.gen_range(-y_span..y_span);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOR,
                custom_size: Some(ENEMY_SIZE),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(SpriteSize(ENEMY_SIZE));
}
