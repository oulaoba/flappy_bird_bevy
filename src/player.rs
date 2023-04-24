use bevy::{
    prelude::{
        Audio, Commands, Input, IntoSystemAppConfig, IntoSystemConfigs, KeyCode, OnEnter, OnUpdate,
        Plugin, Query, Res, ResMut, Transform, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::{Timer, TimerMode},
};

use crate::{
    components::{Movable, Player, PlayerAnimationTimer, Velocity},
    constants::{
        GRAVITY_VELOCITY, PLAYER_Y_MAX_UP_VELOCITY, PLAYER_Y_MAX_VELOCITY, PLAYER_Y_UP_PIXEL,
        TIME_STEP,
    },
    resource::{GameData, StaticAssets, WinSize},
    state::GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            (input_key_system, bird_automatic_system).in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(spawn_bird_system.in_schedule(OnEnter(GameState::InGame)));
    }
}

/// 产生玩家
fn spawn_bird_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    static_assets: Res<StaticAssets>,
    mut game_data: ResMut<GameData>,
) {
    if !game_data.player_alive() {
        let bird = static_assets.player.clone();
        let (x, y) = (-win_size.width / 4. / 2., win_size.height / 2. / 3.);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: bird,
                transform: Transform {
                    translation: Vec3 { x, y, z: 2. },
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(0),
                ..Default::default()
            },
            Player,
            Velocity { x: 0., y: 0. },
            Movable {
                need_rotation: true,
            },
            PlayerAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        ));
        game_data.alive();
    }
}

/// 游戏中键盘事件系统
fn input_key_system(
    kb: Res<Input<KeyCode>>,
    static_assets: Res<StaticAssets>,
    audio_player: Res<Audio>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    if kb.just_released(KeyCode::Space) {
        let vt = query.get_single_mut();
        // 松开空格后，直接向上20像素，并且给一个向上的速度。
        match vt {
            Ok((mut velocity, mut transform)) => {
                transform.translation.y += PLAYER_Y_UP_PIXEL;
                velocity.y = PLAYER_Y_MAX_UP_VELOCITY;
            }
            _ => (),
        }
        audio_player.play(static_assets.fly_audio.clone());
    }
}

/// 小鸟重力系统
fn bird_automatic_system(mut query: Query<&mut Velocity, (With<Player>, With<Movable>)>) {
    for mut velocity in query.iter_mut() {
        velocity.y = velocity.y - GRAVITY_VELOCITY * TIME_STEP;
        if velocity.y < -PLAYER_Y_MAX_VELOCITY {
            velocity.y = -PLAYER_Y_MAX_VELOCITY;
        }
    }
}
