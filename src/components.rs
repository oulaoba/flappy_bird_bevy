use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};

/// 玩家组件
#[derive(Component)]
pub struct Player;

/// 玩家动画播放计时器
#[derive(Component)]
pub struct PlayerAnimationTimer(pub Timer);

impl Default for PlayerAnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

/// 障碍物组件
#[derive(Component)]
pub struct Obstacle;

/// 移动组件
#[derive(Component)]
pub struct Movable {
    /// 移动时是否需要旋转
    pub need_rotation: bool,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            need_rotation: false,
        }
    }
}

/// 速度组件
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

/// 分数显示组件
#[derive(Component)]
pub struct DisplayScore;

/// 菜单显示组件
#[derive(Component)]
pub struct DisplayMenu;

/// 地面组件
#[derive(Component)]
pub struct Ground(pub f32);

/// 游戏结束组件
#[derive(Component)]
pub struct DisplayGameOver;
