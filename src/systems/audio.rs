use macroquad::audio::{Sound, load_sound_from_bytes, play_sound_once};

pub struct SoundManager {
    pub transition_sound: Option<Sound>,
    pub bounce_sound: Option<Sound>,
}

impl SoundManager {
    pub async fn new() -> Self {
        let transition_data = include_bytes!("../../assets/transition.wav");
        let bounce_data = include_bytes!("../../assets/bounce.wav");

        let transition = load_sound_from_bytes(transition_data).await.ok();
        let bounce = load_sound_from_bytes(bounce_data).await.ok();

        Self {
            transition_sound: transition,
            bounce_sound: bounce,
        }
    }

    pub fn play_transition(&self) {
        if let Some(s) = &self.transition_sound {
            play_sound_once(s);
        }
    }

    pub fn play_bounce(&self) {
        if let Some(s) = &self.bounce_sound {
            play_sound_once(s);
        }
    }
}
