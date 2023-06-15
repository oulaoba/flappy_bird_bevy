# Rust + Bevy 实现的 Flappy Bird 游戏

## 简介
一个使用 bevy 引擎复刻的 Flappy Bird 经典小游戏。

简单介绍一下包含的内容：

- 游戏状态管理 Menu、InGame、Paused、GameOver。
- 小鸟碰撞检测。
- 地面移动。
- 小鸟飞翔动画。
- 小鸟飞行方向变化。
- 小鸟重力系统。
- 障碍物随机生成。

通过空格向上飞行。
按 P 暂停游戏，按 R 恢复游戏。

> ## 新增 wasm 运行环境
```
1⃣️：安装 wasm-server-runner
cargo install wasm-server-runner

2⃣️：cargo.toml 文件新增配置
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"

3⃣️：使用 wasm-server-runner 生成 wasm
cargo run --target wasm32-unknown-unknown  

4⃣️：运行 wasm
wasm-server-runner target/wasm32-unknown-unknown/debug/flappy_bird_bevy.wasm

5⃣️：打开服务器

```


## 代码结构
```
·
├── assets/
│   ├──audios/
│   ├──fonts/
│   └──images/
├── src/
│   ├── build.rs
│   ├── components.rs
│   ├── constants.rs
│   ├── main.rs
│   ├── obstacle.rs
│   ├── player.rs
│   ├── resource.rs
│   └── state.rs
├── Cargo.lock
└── Cargo.toml
```

- assets/audios 声音资源文件。
- assets/fonts 字体资源文件。
- assets/images 图片资源文件。
- build.rs 构建之前执行的脚本文件。
- components.rs 游戏组件定义。
- constants.rs 负责存储游戏中用到的常量。
- main.rs 负责游戏的逻辑、插件交互、等内容。
- obstacle.rs 障碍物生成、初始化。
- player.rs 玩家角色插件，生成、移动、键盘处理的实现。
- resource.rs 游戏资源定义。
- state.rs 游戏状态管理。


## about me 
目前失业，在家学习 rust 。

我的 [bilibili](https://space.bilibili.com/259260787),我的 [博客园](https://www.cnblogs.com/SantiagoZhang)。


[Rust官网](https://www.rust-lang.org/)
[Rust 中文社区](https://rustcc.cn/)