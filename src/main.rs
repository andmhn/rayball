mod components;
mod constants;
mod game;
mod physics;

use constants::*;
use game::{Game, SoundManager};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_W as i32, WINDOW_H as i32)
        .title("rayball")
        .build();

    let audio = RaylibAudio::init_audio_device();
    let sounds = SoundManager::new(audio.as_ref().ok());

    let mut game = Game::new(sounds);

    rl.set_target_fps(120);

    while !rl.window_should_close() {
        game.update(&rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);
        game.draw(&mut d);
    }
}
