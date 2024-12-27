use std::error::Error;
use std::path::Path;

use crate::lyrics::{Lyric, load_lyrics};
use crate::video::{generate_frames, generate_video};
use crate::audio::load_audio;

struct Config {
    width: u32,
    height: u32,
    fps: u32,
    font_height: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config {
        width: 1920,
        height: 1080,
        fps: 30,
        font_height: 60.0,
    };

    let font = load_font()?;
    let audio_path = "../audio.mp3"; // You can change this to a custom path
    let (source, total_duration) = load_audio(audio_path)?;

    // Define lyrics (you would replace this with actual lyrics)
    let lyrics = vec![
        Lyric {
            text: "First line of lyrics".to_string(),
            start_time: 0.0,
            end_time: 2.0,
        },
        Lyric {
            text: "Second line of lyrics".to_string(),
            start_time: 2.0,
            end_time: 4.0,
        },
        // Add more lyrics...
    ];

    let total_frames = (total_duration * config.fps as f32) as u32;

    println!("🖼️ Total duration: {} seconds", total_duration);
    println!("🎞️ Total frames: {}", total_frames);

    generate_frames(&config, &font, &lyrics, total_frames)?;
    generate_video(&config, audio_path, total_duration)?;

    Ok(())
}
