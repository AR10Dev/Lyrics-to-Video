use rodio::{Decoder, Source};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn load_audio(audio_path: &str) -> Result<(Decoder<File>, f32), Box<dyn Error>> {
    println!("🎵 Loading audio file...");
    if !Path::new(audio_path).exists() {
        return Err(format!("Audio file not found: {}", audio_path).into());
    }
    let audio_file = File::open(audio_path)?;
    let source: Decoder<File> = Decoder::new(audio_file)?;
    let total_duration = source.total_duration().unwrap().as_secs_f32();
    Ok((source, total_duration))
}
