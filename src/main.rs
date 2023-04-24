use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
    window::{Window, WindowPlugin, WindowPosition},
};
use obstacle::ObstaclePlugin;

use components::{DisplayScore, Ground, Movable, Obstacle, Player, PlayerAnimationTimer, Velocity};
use constants::*;
use player::PlayerPlugin;
use resource::{GameData, StaticAssets, WinSize};
use state::{GameState, StatesPlugin};

mod components;
mod constants;
mod obstacle;
mod player;
mod resource;
mod state;

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb_u8(205, 201, 201)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Bird".to_owned(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                position: WindowPosition::At(IVec2::new(2282, 0)),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_system(system_startup.on_startup())
        .add_plugin(StatesPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ObstaclePlugin)
        .add_systems(
            (
                score_display_update_system,
                player_animation_system,
                player_score_system,
                movable_system,
                ground_move_system,
                player_collision_check_system,
            )
                .in_set(OnUpdate(GameState::InGame)),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

/// 玩家碰撞检测系统
fn player_collision_check_system(
    win_size: Res<WinSize>,
    static_assets: Res<StaticAssets>,
    audio_player: Res<Audio>,
    mut next_state: ResMut<NextState<GameState>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let player_result = player_query.get_single();
    match player_result {
        Ok((_, player_tf)) => {
            let mut is_collision = false;
            // 先进行边缘碰撞检测
            if player_tf.translation.y >= win_size.height / 2.
                || player_tf.translation.y <= -(win_size.height / 2. - GROUND_IMG_SIZE.1)
            {
                is_collision = true;
            }

            for (_, obstacle_tf) in obstacle_query.iter() {
                let collision = collide(
                    player_tf.translation,
                    Vec2 {
                        x: BIRD_IMG_SIZE.0,
                        y: BIRD_IMG_SIZE.1,
                    },
                    obstacle_tf.translation,
                    Vec2 {
                        x: PIPE_IMG_SIZE.0,
                        y: PIPE_IMG_SIZE.1,
                    },
                );
                if let Some(_) = collision {
                    is_collision = true;
                    break;
                }
            }
            // 判断是否已经发生碰撞
            if is_collision {
                // 增加得分并播放声音
                audio_player.play(static_assets.hit_audio.clone());
                audio_player.play(static_assets.die_audio.clone());
                next_state.set(GameState::GameOver);
            }
        }
        _ => (),
    }
}

/// 玩家得分检测
fn player_score_system(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    static_assets: Res<StaticAssets>,
    audio_player: Res<Audio>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let player_result = player_query.get_single();
    match player_result {
        Ok((_, player_tf)) => {
            let mut need_add_score = false;
            for (entity, obstacle_tf) in obstacle_query.iter() {
                // 鸟的 尾巴通过管道的右边缘
                if player_tf.translation.x - BIRD_IMG_SIZE.0 / 2.
                    > obstacle_tf.translation.x + PIPE_IMG_SIZE.0 / 2.
                {
                    // 通过的话，将需要得分记为 true 并销毁管道
                    need_add_score = true;
                    commands.entity(entity).despawn();
                }
            }
            // 判断是否需要增加得分
            if need_add_score {
                // 增加得分并播放声音
                game_data.add_score();
                audio_player.play(static_assets.point_audio.clone());
                game_data.call_obstacle_spawn();
            }
        }
        _ => (),
    }
}

/// 移动系统
///
/// * 不考虑正负值，只做加法，需要具体的实体通过移动的方向自行考虑正负值
fn movable_system(
    mut query: Query<(&mut Transform, &Velocity, &Movable), (With<Movable>, With<Velocity>)>,
) {
    for (mut transform, velocity, movable) in query.iter_mut() {
        let x = velocity.x * TIME_STEP;
        let y = velocity.y * TIME_STEP;
        transform.translation.x += x;
        transform.translation.y += y;
        // 判断是否需要旋转
        if movable.need_rotation {
            if velocity.y > 0. {
                transform.rotation = Quat::from_rotation_z(velocity.y / PLAYER_Y_MAX_UP_VELOCITY);
            } else {
                transform.rotation = Quat::from_rotation_z(velocity.y / PLAYER_Y_MAX_VELOCITY);
            };
        }
    }
}

/// 地面移动组件
fn ground_move_system(mut query: Query<(&mut Transform, &mut Ground)>) {
    let result = query.get_single_mut();
    match result {
        Ok((mut transform, mut ground)) => {
            ground.0 += 1.;
            transform.translation.x = -ground.0;
            ground.0 = ground.0 % GROUND_ITEM_SIZE;
        }
        _ => (),
    }
}

/// 角色动画系统
fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut texture_atlas_sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            let next_index = (texture_atlas_sprite.index + 1) % BIRD_ANIMATION_LEN;
            texture_atlas_sprite.index = next_index;
        }
    }
}

/// 分数更新系统
fn score_display_update_system(
    game_data: Res<GameData>,
    mut query: Query<&mut Text, With<DisplayScore>>,
) {
    for mut text in &mut query {
        text.sections[1].value = game_data.get_score().to_string();
    }
}

fn system_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Query<&Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let game_data = GameData::new();
    commands.insert_resource(game_data);

    let window = windows.single();
    let (window_w, window_h) = (window.width(), window.height());
    let win_size = WinSize {
        width: window_w,
        height: window_h,
    };
    commands.insert_resource(win_size);

    let player_handle = asset_server.load(BIRD_IMG_PATH);

    // 将 player_handle 加载的图片，用 BIRD_IMG_SIZE 的大小，按照 1 列，3 行，切图。
    let texture_atlas =
        TextureAtlas::from_grid(player_handle, Vec2::from(BIRD_IMG_SIZE), 1, 3, None, None);
    let player = texture_atlases.add(texture_atlas);

    let background = asset_server.load(BACKGROUND_IMG_PATH);
    let pipe = asset_server.load(PIPE_IMG_PATH);
    let ground = asset_server.load(GROUND_IMG_PATH);
    let fly_audio = asset_server.load(FLAY_AUDIO_PATH);
    let die_audio = asset_server.load(DIE_AUDIO_PATH);
    let point_audio = asset_server.load(POINT_AUDIO_PATH);
    let hit_audio = asset_server.load(HIT_AUDIO_PATH);
    let kenney_future_font = asset_server.load(KENNEY_FUTURE_FONT_PATH);

    let static_assets = StaticAssets {
        player,
        background,
        pipe,
        ground,
        fly_audio,
        die_audio,
        point_audio,
        hit_audio,
        kenney_future_font,
    };
    commands.insert_resource(static_assets);

    let (background_w, background_h) = BACKGROUND_IMG_SIZE;
    let (ground_w, ground_h) = GROUND_IMG_SIZE;
    commands.spawn(SpriteBundle {
        texture: asset_server.load(BACKGROUND_IMG_PATH),
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: background_w * 2.,
                y: background_h,
            }),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 {
                x: 0.,
                y: ground_h / 2.,
                z: 1.,
            },
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(GROUND_IMG_PATH),
            sprite: Sprite {
                custom_size: Some(Vec2 {
                    x: ground_w * 2.,
                    y: ground_h,
                }),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: window_h / 2. - background_h - ground_h / 2.,
                    z: 4.,
                },
                ..Default::default()
            },

            ..Default::default()
        },
        Ground(GROUND_ITEM_SIZE),
    ));
}
