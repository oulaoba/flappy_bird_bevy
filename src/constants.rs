/// 小鸟图片路径
pub const BIRD_IMG_PATH: &str = "images/bird_columns.png";
/// 小鸟图片大小
pub const BIRD_IMG_SIZE: (f32, f32) = (34., 24.);
/// 小鸟动画帧数
pub const BIRD_ANIMATION_LEN: usize = 3;

pub const WINDOW_WIDTH: f32 = 576.;
pub const WINDOW_HEIGHT: f32 = 624.;

/// 背景图片路径
pub const BACKGROUND_IMG_PATH: &str = "images/background.png";
/// 背景图片大小
pub const BACKGROUND_IMG_SIZE: (f32, f32) = (288., 512.);
/// 地面图片路径
pub const GROUND_IMG_PATH: &str = "images/ground.png";
/// 地面图片大小
pub const GROUND_IMG_SIZE: (f32, f32) = (336., 112.);
/// 一个单位地面的大小
pub const GROUND_ITEM_SIZE: f32 = 48.;
/// 管道图片路径
pub const PIPE_IMG_PATH: &str = "images/pipe.png";
/// 管道图片大小
pub const PIPE_IMG_SIZE: (f32, f32) = (52., 320.);
/// 飞翔声音路径
pub const FLAY_AUDIO_PATH: &str = "audios/wing.ogg";
/// 得分声音
pub const POINT_AUDIO_PATH: &str = "audios/point.ogg";
/// 死亡声音
pub const DIE_AUDIO_PATH: &str = "audios/die.ogg";
/// 被撞击声音
pub const HIT_AUDIO_PATH: &str = "audios/hit.ogg";
/// kenney future 字体路径
pub const KENNEY_FUTURE_FONT_PATH: &str = "fonts/KenneyFuture.ttf";

/// x 轴前进速度
pub const SPAWN_OBSTACLE_TICK: f32 = 4.;
/// x 轴前进速度
pub const PLAYER_X_MAX_VELOCITY: f32 = 48.;
/// y 轴最大上升速度
pub const PLAYER_Y_MAX_UP_VELOCITY: f32 = 20.;
/// y 轴每次上升像素
pub const PLAYER_Y_UP_PIXEL: f32 = 10.;
/// y 轴最大下落速度
pub const PLAYER_Y_MAX_VELOCITY: f32 = 200.;
/// y 轴下落加速度，每秒增加
pub const GRAVITY_VELOCITY: f32 = 80.;
/// 步长 (帧数)
pub const TIME_STEP: f32 = 1. / 60.;

/// 最大通过空间
pub const GAP_MAX: f32 = 300.;
/// 最小通过空间
pub const GAP_MIN: f32 = 50.;
