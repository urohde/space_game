use bevy::math::Vec3Swizzles;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use components::{Enemy, FromPlayer, Laser, Movable, SpriteSize, Velocity};
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod components;
mod enemy;
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
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin))
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup_system)
        .add_systems(
            Update,
            (movable_system, player_lazer_enemy_collision_system),
        )
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
                commands.entity(entity).despawn();
            }
        }
    }
}

fn player_lazer_enemy_collision_system(
    mut commands: Commands,
    lasers: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemies: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (laser_entity, laser_transform, laser_size) in lasers.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }

        let laser_scale = Vec2::from(laser_transform.scale.xy());

        for (enemy_entity, enemy_transform, enemy_size) in enemies.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&laser_entity)
            {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_transform.scale.xy());

            match collide(
                laser_transform.translation,
                laser_size.0 * laser_scale,
                enemy_transform.translation,
                enemy_size.0 * enemy_scale,
            ) {
                Some(_) => {
                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    commands.entity(laser_entity).despawn();
                    despawned_entities.insert(laser_entity);
                }
                _ => (),
            }
        }
    }
}
