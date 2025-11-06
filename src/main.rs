use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_enemy)
        .add_systems(Update, enemy_movement)
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Sprite {
            image: asset_server.load("sprite/ball_blue_large.png"),
            ..default()
        },
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub const NUMBER_OF_ENEMIES: usize = 4;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x: f32 = random::<f32>() * window.width();
        let random_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            Sprite {
                image: asset_server.load("sprite/ball_red_large.png"),
                ..default()
            },
            Transform::from_xyz(random_x, random_y, 0.0),
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));

        //  commands.spawn((
        //      Sprite {
        //          image: asset_server.load("sprite/ball_red_large.png"),
        //          ..default()
        //      },
        //      Transform::from_xyz(random_x, random_y, 0.0),
        //      Enemy {
        //          direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        //      },
        //  ));
    }
}

pub const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            println!("A pressed");
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            println!("D pressed");
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            println!("W pressed");
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            println!("S pressed");
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();

            transform.translation += direction * PLAYER_SPEED * time.delta_secs();
        }
    }
}

pub const PLAYER_SIZE: f32 = 64.0; //This is the player sprite size
                                   //
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let window = window_query.single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0; // 32.0
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        //Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        //Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub const ENEMY_SPEED: f32 = 200.0;

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}
