extern crate sfml;

use crate::app::*;
use sfml::audio::*;

static sample_rate: u32 = 44100;

pub struct SoundCtx {
    buffer: Vec<u8>,
}

impl SoundCtx {
    pub fn new() -> SoundCtx {
        let mut ctx = SoundCtx {
            buffer: Vec::new(),
        };
        ctx.buffer.reserve(sample_rate as usize);
        ctx
    }
    pub fn square(&mut self, freq: f32) {
        let tps = sample_rate as f32 / freq;
        for i in 0..self.buffer.len() {
            if i as f32 % (tps / 2.) == 0. {
                self.buffer[i] = 50;
            } else {
                self.buffer[i] = 0;
            }
        }
    }
}

pub fn test(ctx: &mut SoundCtx) {
    ctx.square(480.);
    let soundbuffer = &*SoundBuffer::from_memory(ctx.buffer.as_slice()).unwrap();
    let mut sound = Sound::with_buffer(soundbuffer);
    sound.play();
    while sound.status() == SoundStatus::Playing {
    }
}

/*
pub fn whistle_rocket(ctx: &mut SoundCtx,firework: &Firework) {
}
*/
