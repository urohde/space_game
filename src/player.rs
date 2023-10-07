use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    components::{Movable, Player, Velocity},
    WinSize
};

const PLAYER_SIZE: Vec2 = Vec2::new(50., 50.);
const PLAYER_COLOR: Color = Color::RED;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player_system)
            .add_systems(FixedUpdate, player_fire_system)
            .add_systems(Update, player_keybord_event_system);
    }
}

fn spawn_player_system(mut commands: Commands, win_size: Res<WinSize>) {
    let bottom = -win_size.h / 2.;
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, bottom + PLAYER_SIZE.y / 2.0 + 5.0, 10.0),
                ..default()
            },
            ..default()
        },))
        .insert(Player)
        .insert(Movable {
            auto_despawn: false,
        })
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                    ..default()
                })
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: 1. });
        }
    }
}

fn player_keybord_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut v) = query.get_single_mut() {
        v.x = if kb.pressed(KeyCode::Right) {
            1.0
        } else if kb.pressed(KeyCode::Left) {
            -1.0
        } else {
            0.0
        };
    }
}

