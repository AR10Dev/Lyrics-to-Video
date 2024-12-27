use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;

mod lyrics;
mod video;
mod audio;

use lyrics::Lyric;
use video::{generate_frames, generate_video};
use audio::load_audio;

#[derive(Deserialize)]
struct Config {
    width: u32,
    height: u32,
    fps: u32,
    font_height: f32,
    audio_path: String,
    font_path: String,
    output_directory: String,
    lyrics: Vec<Lyric>,
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;

    let font = load_font(&config.font_path)?;
    let (source, total_duration) = load_audio(&config.audio_path)?;

    let total_frames = (total_duration * config.fps as f32) as u32;

    println!("🖼️ Total duration: {} seconds", total_duration);
    println!("🎞️ Total frames: {}", total_frames);

    generate_frames(
        config.width,
        config.height,
        config.fps,
        config.font_height,
        &font,
        &config.lyrics,
        total_frames,
    )?;
    generate_video(config.fps, &config.audio_path, total_duration)?;

    Ok(())
}

fn load_font(font_path: &str) -> Result<ab_glyph::FontVec, Box<dyn Error>> {
    let font_data = std::fs::read(font_path)?;
    let font = ab_glyph::FontVec::try_from_vec(font_data)?;
    Ok(font)
}
