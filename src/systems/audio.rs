use raylib::prelude::{RaylibAudio, Sound};

pub struct SoundManager<'a> {
    pub transition_sound: Option<Sound<'a>>,
    pub bounce_sound: Option<Sound<'a>>,
}

impl<'a> SoundManager<'a> {
    pub fn new(audio_handle: Option<&'a RaylibAudio>) -> Self {
        match audio_handle {
            Some(h) => Self {
                transition_sound: h.new_sound("assets/transition.wav").ok(),
                bounce_sound: h.new_sound("assets/bounce.wav").ok(),
            },
            None => Self {
                transition_sound: None,
                bounce_sound: None,
            },
        }
    }

    pub fn play_transition(&self) {
        if let Some(s) = &self.transition_sound {
            s.play();
        }
    }

    pub fn play_bounce(&self) {
        if let Some(s) = &self.bounce_sound {
            s.play();
        }
    }
}
