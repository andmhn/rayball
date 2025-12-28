use raylib::prelude::{RaylibAudio, Sound};

pub struct SoundManager<'a> {
    pub drop_sound: Option<Sound<'a>>,
    pub bounce_sound: Option<Sound<'a>>,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio_handle: Option<&'a RaylibAudio>) -> Self {
        match audio_handle {
            Some(h) => Self {
                drop_sound: h.new_sound("assets/dropped.wav").ok(),
                bounce_sound: h.new_sound("assets/bounce.wav").ok(),
            },
            None => Self {
                drop_sound: None,
                bounce_sound: None,
            },
        }
    }

    pub fn play_drop(&self) {
        if let Some(s) = &self.drop_sound {
            s.play();
        }
    }

    pub fn play_bounce(&self) {
        if let Some(s) = &self.bounce_sound {
            s.play();
        }
    }
}
