#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod components;
mod constants;
mod game;
mod systems;

use std::sync::OnceLock;

use constants::*;
use game::Game;
use macroquad::prelude::*;
use systems::audio::SoundManager;

static DEFAULT_FONT: OnceLock<Font> = OnceLock::new();

#[macroquad::main(window_conf)]
async fn main() {
    init_logger();

    let font = load_ttf_font_from_bytes(include_bytes!("../assets/Cousine-Regular.ttf"));
    DEFAULT_FONT.set(font.unwrap()).unwrap();

    let sounds = SoundManager::new();
    let mut game = Game::new(sounds.await);
    log::info!("Game started successfully");

    loop {
        let start_frame = get_time();

        game.update();
        clear_background(BG_COLOR);
        game.draw();

        if is_key_down(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        next_frame().await;
        limit_framerate(start_frame);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "rayball".to_owned(),
        window_width: 1100,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

const FRAME_TIME_TARGET: f64 = 1.0 / 120.; // FPS

fn limit_framerate(start_frame: f64) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let elapsed = get_time() - start_frame;
        if elapsed < FRAME_TIME_TARGET {
            let sleep_time = (FRAME_TIME_TARGET - elapsed) * 1000.0;
            std::thread::sleep(std::time::Duration::from_millis(sleep_time as u64));
        }
    }
}

fn init_logger() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use simplelog::*;
        let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];

        loggers.push(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ));

        if let Ok(file) = std::fs::File::create("rayball.log") {
            loggers.push(WriteLogger::new(LevelFilter::Info, Config::default(), file));
        }

        CombinedLogger::init(loggers).unwrap();
    }
}
