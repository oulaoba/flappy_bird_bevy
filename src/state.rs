use bevy::{
    prelude::{
        Color, Commands, Entity, Input, IntoSystemAppConfig, IntoSystemConfig, KeyCode, NextState,
        OnEnter, OnExit, OnUpdate, Plugin, Query, Res, ResMut, States, Transform, Vec3, With,
    },
    text::{Text, Text2dBundle, TextAlignment, TextSection, TextStyle},
};

use crate::{
    components::{DisplayGameOver, DisplayMenu, DisplayScore, Obstacle, Player},
    constants::GROUND_IMG_SIZE,
    resource::{GameData, StaticAssets, WinSize},
};

#[derive(Debug, Default, States, PartialEq, Eq, Clone, Hash)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    Paused,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            //菜单状态
            .add_system(menu_display_system.in_schedule(OnEnter(GameState::Menu)))
            .add_system(enter_game_system.in_set(OnUpdate(GameState::Menu)))
            .add_system(exit_menu.in_schedule(OnExit(GameState::Menu)))
            // 暂停状态
            .add_system(enter_paused_system.in_schedule(OnEnter(GameState::Paused)))
            .add_system(paused_input_system.in_set(OnUpdate(GameState::Paused)))
            .add_system(paused_exit_system.in_schedule(OnExit(GameState::Paused)))
            // 游戏中状态
            .add_system(in_game_display_system.in_schedule(OnEnter(GameState::InGame)))
            .add_system(in_game_input_system.in_set(OnUpdate(GameState::InGame)))
            .add_system(exit_game_system.in_schedule(OnExit(GameState::InGame)))
            // 游戏结束状态
            .add_system(game_over_enter_system.in_schedule(OnEnter(GameState::GameOver)))
            .add_system(in_game_over_system.in_set(OnUpdate(GameState::GameOver)))
            .add_system(game_over_exit_system.in_schedule(OnExit(GameState::GameOver)));
    }
}

//// 进入菜单页面
fn menu_display_system(mut commands: Commands, static_assets: Res<StaticAssets>) {
    let font = static_assets.kenney_future_font.clone();
    let common_style = TextStyle {
        font: font.clone(),
        font_size: 32.,
        color: Color::BLUE,
        ..Default::default()
    };
    let special_style = TextStyle {
        font: font.clone(),
        font_size: 38.,
        color: Color::RED,
        ..Default::default()
    };

    let align = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::new("PRESS \r\n".to_owned(), common_style.clone()),
                TextSection::new(" SPACE \r\n".to_owned(), special_style.clone()),
                TextSection::new("START GAME!\r\n".to_owned(), common_style.clone()),
                TextSection::new(" P \r\n".to_owned(), special_style.clone()),
                TextSection::new("PAUSED GAME!\r\n".to_owned(), common_style.clone()),
            ])
            .with_alignment(align),
            transform: Transform {
                translation: Vec3::new(0., 0., 4.),
                ..Default::default()
            },
            ..Default::default()
        },
        DisplayMenu,
    ));
}

//// 进入游戏显示系统
fn in_game_display_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    static_assets: Res<StaticAssets>,
) {
    let font = static_assets.kenney_future_font.clone();
    let common_style = TextStyle {
        font: font.clone(),
        font_size: 32.,
        color: Color::BLUE,
        ..Default::default()
    };
    let special_style = TextStyle {
        font: font.clone(),
        font_size: 38.,
        color: Color::RED,
        ..Default::default()
    };
    let y = -(win_size.height / 2. - GROUND_IMG_SIZE.1 + special_style.font_size * 1.5);
    let align = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::new("SCORE: ".to_owned(), common_style),
                TextSection::new("0".to_owned(), special_style),
            ])
            .with_alignment(align),
            transform: Transform {
                translation: Vec3::new(0., y, 6.),
                ..Default::default()
            },
            ..Default::default()
        },
        DisplayScore,
    ));
}

/// 进入游戏
fn enter_game_system(kb: Res<Input<KeyCode>>, mut state: ResMut<NextState<GameState>>) {
    if kb.just_released(KeyCode::Space) {
        state.set(GameState::InGame)
    }
}

/// 退出游戏
fn exit_game_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<DisplayScore>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
/// 退出菜单
fn exit_menu(mut commands: Commands, query: Query<Entity, (With<Text>, With<DisplayMenu>)>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// 进入暂停状态下运行的系统
pub fn enter_paused_system(mut commands: Commands, static_assets: Res<StaticAssets>) {
    // 字体引入
    let font = static_assets.kenney_future_font.clone();
    let common_style = TextStyle {
        font: font.clone(),
        font_size: 32.,
        color: Color::BLUE,
        ..Default::default()
    };
    let special_style = TextStyle {
        font: font.clone(),
        font_size: 38.,
        color: Color::RED,
        ..Default::default()
    };

    let align = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::new("PAUSED  \r\n".to_owned(), common_style.clone()),
                TextSection::new(" R \r\n".to_owned(), special_style.clone()),
                TextSection::new("RETURN GAME!".to_owned(), common_style.clone()),
            ])
            .with_alignment(align),
            transform: Transform {
                translation: Vec3::new(0., 0., 4.),
                ..Default::default()
            },
            ..Default::default()
        },
        DisplayMenu,
    ));
}

/// 暂停状态状态下的键盘监听系统
pub fn paused_input_system(kb: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if kb.pressed(KeyCode::R) {
        next_state.set(GameState::InGame);
    }
}

/// 退出暂停状态时执行的系统
pub fn paused_exit_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<DisplayMenu>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// 游戏中监听暂停
pub fn in_game_input_system(kb: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if kb.pressed(KeyCode::P) {
        next_state.set(GameState::Paused);
    }
}

/// 游戏结束状态下运行的系统
pub fn game_over_enter_system(
    mut commands: Commands,
    game_data: Res<GameData>,
    static_assets: Res<StaticAssets>,
) {
    // 字体引入
    let font = static_assets.kenney_future_font.clone();
    let common_style = TextStyle {
        font: font.clone(),
        font_size: 32.,
        color: Color::BLUE,
        ..Default::default()
    };
    let special_style = TextStyle {
        font: font.clone(),
        font_size: 38.,
        color: Color::RED,
        ..Default::default()
    };

    let align = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::new(
                    "GAME OVER !  \r\n You got ".to_owned(),
                    common_style.clone(),
                ),
                TextSection::new(game_data.get_score().to_string(), special_style.clone()),
                TextSection::new(" score. \r\n  ".to_owned(), common_style.clone()),
                TextSection::new("SPACE ".to_owned(), special_style.clone()),
                TextSection::new("RESTART GAME! \r\n".to_owned(), common_style.clone()),
                TextSection::new("M ".to_owned(), special_style.clone()),
                TextSection::new("TO MENU".to_owned(), common_style.clone()),
            ])
            .with_alignment(align),
            transform: Transform {
                translation: Vec3::new(0., 80., 4.),
                ..Default::default()
            },
            ..Default::default()
        },
        DisplayGameOver,
    ));
}

/// 退出游戏状态时执行的系统
pub fn game_over_exit_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<DisplayGameOver>)>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    player_query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in obstacle_query.iter() {
        commands.entity(entity).despawn();
    }
    for entity in player_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// 退出游戏状态监听
pub fn in_game_over_system(
    kb: Res<Input<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    game_data.death();
    if kb.pressed(KeyCode::M) {
        next_state.set(GameState::Menu);
    } else if kb.pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }
}
