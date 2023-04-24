use bevy::{
    prelude::{AudioSource, Handle, Image, Resource},
    sprite::TextureAtlas,
    text::Font,
};

/// 游戏数据资源
#[derive(Resource)]
pub struct GameData {
    score: u8,
    alive: bool,
    need_add_obstacle: bool,
}
impl GameData {
    pub fn new() -> Self {
        Self {
            score: 0,
            alive: false,
            need_add_obstacle: false,
        }
    }

    pub fn need_spawn_obstacle(&self) -> bool {
        self.need_add_obstacle
    }

    pub fn obstacle_call_back(&mut self) {
        self.need_add_obstacle = false;
    }

    pub fn call_obstacle_spawn(&mut self) {
        self.need_add_obstacle = true;
    }

    pub fn alive(&mut self) {
        self.alive = true;
    }

    pub fn death(&mut self) {
        self.alive = false;
        self.score = 0;
    }

    pub fn get_score(&self) -> u8 {
        self.score
    }

    pub fn add_score(&mut self) {
        self.score += 1;
    }

    pub fn player_alive(&self) -> bool {
        self.alive
    }
}

/// 窗口大小资源
#[derive(Resource)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

/// 静态资源
#[derive(Resource)]
pub struct StaticAssets {
    /* 图片 */
    /// 玩家动画
    pub player: Handle<TextureAtlas>,
    /// 管道图片
    pub pipe: Handle<Image>,
    /// 背景图片
    pub background: Handle<Image>,
    /// 地面图片
    pub ground: Handle<Image>,

    /* 声音 */
    /// 飞行声音
    pub fly_audio: Handle<AudioSource>,
    /// 死亡声音
    pub die_audio: Handle<AudioSource>,
    /// 得分声音
    pub point_audio: Handle<AudioSource>,
    /// 被撞击声音
    pub hit_audio: Handle<AudioSource>,

    /* 字体 */
    /// 游戏字体
    pub kenney_future_font: Handle<Font>,
}
