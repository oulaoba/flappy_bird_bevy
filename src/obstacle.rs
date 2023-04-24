use rand::{thread_rng, Rng};
use std::time::Duration;

use crate::{
    components::{Movable, Obstacle, Velocity},
    constants::{
        BACKGROUND_IMG_SIZE, GAP_MAX, GAP_MIN, GROUND_IMG_SIZE, PIPE_IMG_SIZE,
        PLAYER_X_MAX_VELOCITY, SPAWN_OBSTACLE_TICK,
    },
    resource::{GameData, StaticAssets, WinSize},
    state::GameState,
};

use bevy::{
    prelude::{
        Commands, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin, Res, Transform,
        Vec3,
    },
    sprite::{Sprite, SpriteBundle},
    time::common_conditions::on_timer,
};

/// 障碍物插件
pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(obstacle_init_system.in_schedule(OnEnter(GameState::InGame)))
            .add_system(
                spawn_obstacle_system
                    .in_set(OnUpdate(GameState::InGame))
                    .run_if(on_timer(Duration::from_secs_f32(SPAWN_OBSTACLE_TICK))),
            );
    }
}

/// 障碍物初始化
fn obstacle_init_system(
    mut commands: Commands,
    static_assets: Res<StaticAssets>,
    win_size: Res<WinSize>,
    game_data: Res<GameData>,
) {
    let mut rng = thread_rng();
    // 初始 x 坐标
    let x = win_size.width / 2. + PIPE_IMG_SIZE.0 / 2.;
    // 初始化 管道区域的中心点。因为要排除地面的高度
    let center_y = (win_size.height - BACKGROUND_IMG_SIZE.1) / 2.;
    // 定义合理范围
    let reasonable_y_max = win_size.height / 2. - 100.;
    let reasonable_y_min = -(win_size.height / 2. - 100. - GROUND_IMG_SIZE.1);

    let size = SPAWN_OBSTACLE_TICK * PLAYER_X_MAX_VELOCITY;

    for i in 0..2 {
        let x = x - PIPE_IMG_SIZE.0 - size * i as f32;
        // y轴 随机中心点
        // 随机可通过区域的中心点
        let point_y = rng.gen_range(reasonable_y_min..reasonable_y_max);
        let half_distance = (center_y - point_y).abs() / 2.;

        // 获取得分 ， 并根据得分获取一个随机的可通过区域的大小
        let score = game_data.get_score();
        let max = GAP_MAX - score as f32 / 10.;
        // 不让 max 小于最小值
        // 这里也可以做些其他的判断。改变下别的数据。比如说 让管道的移动速度变快！
        let max = max.max(GAP_MIN);
        let min = GAP_MIN;
        let gap = rng.gen_range(min.min(max)..min.max(max));
        let rand_half_gap = gap * rng.gen_range(0.3..0.7);
        // 通过中心点，可通过区域，以及管道的高来计算 上下两个管道各自中心点的 y 坐标
        let half_pipe = PIPE_IMG_SIZE.1 / 2.;
        let pipe_upper = center_y + half_distance + (rand_half_gap + half_pipe);
        let pipe_down = center_y - half_distance - (gap - rand_half_gap + half_pipe);

        // 下方水管
        commands.spawn((
            SpriteBundle {
                texture: static_assets.pipe.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x,
                        y: pipe_down,
                        z: 2.,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            Velocity {
                x: -PLAYER_X_MAX_VELOCITY,
                y: 0.,
            },
            Movable {
                need_rotation: false,
            },
            Obstacle,
        ));

        // 上方水管
        commands.spawn((
            SpriteBundle {
                texture: static_assets.pipe.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x,
                        y: pipe_upper,
                        z: 2.,
                    },
                    ..Default::default()
                },
                sprite: Sprite {
                    flip_y: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            Velocity {
                x: -PLAYER_X_MAX_VELOCITY,
                y: 0.,
            },
            Movable {
                need_rotation: false,
            },
            Obstacle,
        ));
    }
}

fn spawn_obstacle_system(
    mut commands: Commands,
    static_assets: Res<StaticAssets>,
    win_size: Res<WinSize>,
    game_data: Res<GameData>,
) {
    let mut rng = thread_rng();
    // 初始 x 坐标
    let x = win_size.width / 2. + PIPE_IMG_SIZE.0 / 2.;
    // 初始化 管道区域的中心点。因为要排除地面的高度
    let center_y = (win_size.height - BACKGROUND_IMG_SIZE.1) / 2.;

    // y轴 随机中心点
    // 定义合理范围
    let reasonable_y_max = win_size.height / 2. - 100.;
    let reasonable_y_min = -(win_size.height / 2. - 100. - GROUND_IMG_SIZE.1);
    // 随机可通过区域的中心点
    let point_y = rng.gen_range(reasonable_y_min..reasonable_y_max);
    let half_distance = (center_y - point_y).abs() / 2.;

    // 获取得分 ， 并根据得分获取一个随机的可通过区域的大小
    let score = game_data.get_score();
    let max = GAP_MAX - score as f32 / 10.;
    // 不让 max 小于最小值
    // 这里也可以做些其他的判断。改变下别的数据。比如说 让管道的移动速度变快！
    let max = max.max(GAP_MIN);
    let min = GAP_MIN;
    let gap = rng.gen_range(min.min(max)..min.max(max));
    let rand_half_gap = gap * rng.gen_range(0.3..0.7);
    // 通过中心点，可通过区域，以及管道的高来计算 上下两个管道各自中心点的 y 坐标
    let half_pipe = PIPE_IMG_SIZE.1 / 2.;
    let pipe_upper = center_y + half_distance + (rand_half_gap + half_pipe);
    let pipe_down = center_y - half_distance - (gap - rand_half_gap + half_pipe);

    // 下方水管
    commands.spawn((
        SpriteBundle {
            texture: static_assets.pipe.clone(),
            transform: Transform {
                translation: Vec3 {
                    x,
                    y: pipe_down,
                    z: 2.,
                },
                ..Default::default()
            },
            ..Default::default()
        },
        Velocity {
            x: -PLAYER_X_MAX_VELOCITY,
            y: 0.,
        },
        Movable {
            need_rotation: false,
        },
        Obstacle,
    ));

    // 上方水管
    commands.spawn((
        SpriteBundle {
            texture: static_assets.pipe.clone(),
            transform: Transform {
                translation: Vec3 {
                    x,
                    y: pipe_upper,
                    z: 2.,
                },
                ..Default::default()
            },
            sprite: Sprite {
                flip_y: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Velocity {
            x: -PLAYER_X_MAX_VELOCITY,
            y: 0.,
        },
        Movable {
            need_rotation: false,
        },
        Obstacle,
    ));
}
