use ab_glyph::{FontRef, PxScale};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Lyric {
    pub text: String,
    pub start_time: f32,
    pub end_time: f32,
}

pub fn load_lyrics(file_path: &str) -> Result<Vec<Lyric>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lyrics = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let text = parts[0].to_string();
            let start_time: f32 = parts[1].parse()?;
            let end_time: f32 = parts[2].parse()?;
            lyrics.push(Lyric {
                text,
                start_time,
                end_time,
            });
        }
    }

    Ok(lyrics)
}
