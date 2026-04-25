// Standalone generator for FocalPoint audio cues.
// rustc gen_cues.rs && ./gen_cues

use std::f32::consts::PI;
use std::fs;
use std::path::Path;

const SAMPLE_RATE: f32 = 44100.0;

fn sine_wave(freq: f32, duration: f32) -> Vec<f32> {
    let num_samples = (duration * SAMPLE_RATE) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let dt = 1.0 / SAMPLE_RATE;
    let mut phase: f32 = 0.0;
    let phase_increment = 2.0 * PI * freq * dt;
    for _ in 0..num_samples {
        samples.push(phase.sin());
        phase += phase_increment;
    }
    samples
}

fn square_wave(freq: f32, duration: f32) -> Vec<f32> {
    let num_samples = (duration * SAMPLE_RATE) as usize;
    let mut samples = Vec::with_capacity(num_samples);
    let dt = 1.0 / SAMPLE_RATE;
    let mut phase: f32 = 0.0;
    let phase_increment = 2.0 * PI * freq * dt;
    for _ in 0..num_samples {
        samples.push(if phase.sin() > 0.0 { 1.0 } else { -1.0 });
        phase += phase_increment;
    }
    samples
}

fn apply_envelope(samples: &mut [f32], attack_ms: f32, sustain_ms: f32, release_ms: f32) {
    let attack_samples = (attack_ms / 1000.0 * SAMPLE_RATE) as usize;
    let sustain_samples = (sustain_ms / 1000.0 * SAMPLE_RATE) as usize;
    let release_samples = (release_ms / 1000.0 * SAMPLE_RATE) as usize;
    for (i, sample) in samples.iter_mut().enumerate() {
        let gain = if i < attack_samples {
            i as f32 / attack_samples.max(1) as f32
        } else if i < attack_samples + sustain_samples {
            1.0
        } else {
            let release_progress =
                (i - attack_samples - sustain_samples) as f32 / release_samples.max(1) as f32;
            (1.0 - release_progress).max(0.0)
        };
        *sample *= gain * 0.7;
    }
}

fn samples_to_pcm16(samples: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(samples.len() * 2);
    for &s in samples {
        let sample = (s * 32767.0).clamp(-32768.0, 32767.0) as i16;
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}

fn write_wav_header(pcm_len: usize) -> Vec<u8> {
    let sample_rate = SAMPLE_RATE as u32;
    let num_channels = 1u16;
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32 / 8;
    let block_align = num_channels * bits_per_sample / 8;
    let mut header = Vec::with_capacity(44);
    header.extend_from_slice(b"RIFF");
    header.extend_from_slice(&(36 + pcm_len as u32).to_le_bytes());
    header.extend_from_slice(b"WAVE");
    header.extend_from_slice(b"fmt ");
    header.extend_from_slice(&16u32.to_le_bytes());
    header.extend_from_slice(&1u16.to_le_bytes());
    header.extend_from_slice(&num_channels.to_le_bytes());
    header.extend_from_slice(&sample_rate.to_le_bytes());
    header.extend_from_slice(&byte_rate.to_le_bytes());
    header.extend_from_slice(&block_align.to_le_bytes());
    header.extend_from_slice(&bits_per_sample.to_le_bytes());
    header.extend_from_slice(b"data");
    header.extend_from_slice(&(pcm_len as u32).to_le_bytes());
    header
}

fn generate_wav(cue_type: &str) -> Vec<u8> {
    let samples = match cue_type {
        "rule-fire" => {
            let mut wave = sine_wave(523.0, 0.08);
            apply_envelope(&mut wave, 5.0, 60.0, 15.0);
            wave
        }
        "achievement" => {
            let mut wave = sine_wave(400.0, 0.2);
            apply_envelope(&mut wave, 10.0, 100.0, 90.0);
            wave
        }
        "intervention-warn" => {
            let mut wave = Vec::new();
            let part1 = {
                let mut w = square_wave(350.0, 0.075);
                apply_envelope(&mut w, 0.0, 75.0, 0.0);
                w
            };
            let part2 = {
                let mut w = square_wave(280.0, 0.075);
                apply_envelope(&mut w, 0.0, 75.0, 0.0);
                w
            };
            wave.extend(part1);
            wave.extend(part2);
            wave
        }
        "focus-start" => {
            let mut wave = sine_wave(220.0, 0.3);
            apply_envelope(&mut wave, 50.0, 150.0, 100.0);
            wave
        }
        "focus-end" => {
            let mut wave = sine_wave(262.0, 0.25);
            apply_envelope(&mut wave, 15.0, 150.0, 85.0);
            wave
        }
        "error" => {
            let mut wave = square_wave(110.0, 0.1);
            apply_envelope(&mut wave, 5.0, 80.0, 15.0);
            wave
        }
        "success" => {
            let mut wave = sine_wave(800.0, 0.18);
            apply_envelope(&mut wave, 5.0, 100.0, 75.0);
            wave
        }
        "mascot-acknowledge" => {
            let mut wave = sine_wave(1047.0, 0.12);
            apply_envelope(&mut wave, 3.0, 85.0, 32.0);
            wave
        }
        _ => sine_wave(440.0, 0.1),
    };
    let pcm = samples_to_pcm16(&samples);
    let mut header = write_wav_header(pcm.len());
    header.extend(pcm);
    header
}

fn main() {
    let cue_types = vec![
        "rule-fire",
        "achievement",
        "intervention-warn",
        "focus-start",
        "focus-end",
        "error",
        "success",
        "mascot-acknowledge",
    ];

    let cues_dir = Path::new("cues");
    fs::create_dir_all(cues_dir).expect("Failed to create cues directory");

    let mut total_size = 0;
    for cue in cue_types {
        let wav = generate_wav(cue);
        let size = wav.len();
        total_size += size;
        let filename = cues_dir.join(format!("{}.wav", cue));
        fs::write(&filename, wav).expect(&format!("Failed to write {}", filename.display()));
        println!("Generated: {} ({} bytes)", filename.display(), size);
    }
    println!("Total audio size: {} bytes ({:.1} KB)", total_size, total_size as f32 / 1024.0);
}
