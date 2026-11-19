//! Procedural sound design for the initial archaeological beta. No external assets.
use gotoo_pixel_engine::{
    Audio, AudioBus, AudioError, NoopAudio, PlaybackId, SoundBank, SoundId, pcm16_mono_wav,
};
use std::f32::consts::TAU;

const RATE: u32 = 44_100;
const AMBIENCE: SoundId = SoundId::new("archive.ambience");
const TICK: SoundId = SoundId::new("archive.tick");
const REJECT: SoundId = SoundId::new("archive.reject");
const SHIFT: SoundId = SoundId::new("archive.shift");
const DISCOVERY: SoundId = SoundId::new("archive.discovery");

#[derive(Clone, Copy)]
pub enum Cue {
    Tick,
    Reject,
    Shift,
    Discovery,
}

pub struct AudioCues {
    bank: SoundBank,
    initialized: bool,
    loop_playback: Option<PlaybackId>,
    failed: bool,
}

impl AudioCues {
    pub fn new() -> Result<Self, AudioError> {
        let mut bank = SoundBank::new();
        bank.insert_wav(AMBIENCE, make_drone()?)?;
        bank.insert_wav(TICK, make_tone(0.065, 560.0, 420.0, 0.14)?)?;
        bank.insert_wav(REJECT, make_tone(0.16, 190.0, 115.0, 0.19)?)?;
        bank.insert_wav(SHIFT, make_tone(0.34, 240.0, 390.0, 0.23)?)?;
        bank.insert_wav(DISCOVERY, make_tone(0.83, 320.0, 790.0, 0.23)?)?;
        Ok(Self {
            bank,
            initialized: false,
            loop_playback: None,
            failed: false,
        })
    }

    pub fn ensure_ambience(&mut self, audio: &mut dyn Audio) {
        if self.initialized {
            return;
        }
        self.initialized = true;
        match self.bank.start_loop_on_bus(audio, AMBIENCE, AudioBus::Music) {
            Ok(playback) => self.loop_playback = Some(playback),
            Err(_) => self.failed = true,
        }
    }

    pub fn play(&mut self, audio: &mut dyn Audio, cue: Cue) {
        let id = match cue {
            Cue::Tick => TICK,
            Cue::Reject => REJECT,
            Cue::Shift => SHIFT,
            Cue::Discovery => DISCOVERY,
        };
        let bus = if matches!(cue, Cue::Tick | Cue::Reject) {
            AudioBus::Ui
        } else {
            AudioBus::Sfx
        };
        if self.bank.play_on_bus(audio, id, bus).is_err() {
            self.failed = true;
        }
    }

    pub fn unavailable(&self) -> bool {
        self.failed
    }
}

/// Deterministic generated WAV with bounded amplitude and a smooth envelope.
fn make_tone(duration: f32, start_hz: f32, end_hz: f32, gain: f32) -> Result<Vec<u8>, AudioError> {
    let len = (RATE as f32 * duration) as usize;
    let mut phase = 0.0_f32;
    let mut samples = Vec::with_capacity(len);
    for i in 0..len {
        let t = i as f32 / len as f32;
        let frequency = start_hz + (end_hz - start_hz) * t;
        phase += TAU * frequency / RATE as f32;
        let attack = (t * 24.0).min(1.0);
        let release = (1.0 - t).powf(2.2);
        let envelope = attack * release;
        let fundamental = phase.sin();
        let upper = (phase * 2.01).sin() * 0.18;
        samples.push(((fundamental + upper) * envelope * gain * i16::MAX as f32) as i16);
    }
    pcm16_mono_wav(RATE, &samples)
}

/// A quiet, two-second integer-frequency drone, seam-safe when looped.
fn make_drone() -> Result<Vec<u8>, AudioError> {
    let len = RATE as usize * 2;
    let mut samples = Vec::with_capacity(len);
    for i in 0..len {
        let t = i as f32 / RATE as f32;
        let sample = (TAU * 55.0 * t).sin() * 0.045
            + (TAU * 110.0 * t).sin() * 0.018
            + (TAU * 165.0 * t).sin() * 0.012;
        samples.push((sample * i16::MAX as f32) as i16);
    }
    pcm16_mono_wav(RATE, &samples)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural_audio_wavs_are_valid_for_gpe() {
        let cues = AudioCues::new().expect("valid procedural sounds");
        let mut audio = NoopAudio::default();
        let mut cues = cues;
        cues.ensure_ambience(&mut audio);
        for cue in [Cue::Tick, Cue::Reject, Cue::Shift, Cue::Discovery] {
            cues.play(&mut audio, cue);
        }
        assert!(!cues.unavailable());
    }

    #[test]
    fn wave_samples_have_bounded_peaks_and_smooth_endpoints() {
        for bytes in [
            make_drone().expect("drone"),
            make_tone(0.3, 440.0, 550.0, 0.20).expect("tone"),
        ] {
            assert!(bytes.starts_with(b"RIFF"));
            assert_eq!(&bytes[8..12], b"WAVE");
            let samples = bytes[44..]
                .chunks_exact(2)
                .map(|pair| i16::from_le_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>();
            let max = samples.iter().map(|sample| i32::from(*sample).abs()).max().unwrap_or(0);
            assert!(max < 15_000, "clip peak must stay well below full scale");
            assert!(i32::from(samples[0]).abs() < 600);
            assert!(i32::from(*samples.last().expect("sample")).abs() < 600);
        }
    }
}
