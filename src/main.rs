use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rodio::{Decoder, Source};
use std::error::Error;
use std::fs::File;
use std::path::Path;

struct Lyric {
    text: String,
    start_time: f32,
    end_time: f32,
}

struct Config {
    width: u32,
    height: u32,
    fps: u32,
    font_height: f32,
}

fn load_font() -> Result<FontRef<'static>, Box<dyn Error>> {
    println!("🔍 Loading font...");
    let font = FontRef::try_from_slice(include_bytes!("../font.ttf"))?;
    Ok(font)
}

fn load_audio(audio_path: &str) -> Result<(Decoder<File>, f32), Box<dyn Error>> {
    println!("🎵 Loading audio file...");
    if !Path::new(audio_path).exists() {
        return Err(format!("Audio file not found: {}", audio_path).into());
    }
    let audio_file = File::open(audio_path)?;
    let source: Decoder<File> = Decoder::new(audio_file)?;
    let total_duration = source.total_duration().unwrap().as_secs_f32();
    Ok((source, total_duration))
}

fn generate_frames(config: &Config, font: &FontRef, lyrics: &[Lyric], total_frames: u32) -> Result<(), Box<dyn Error>> {
    println!("🎨 Generating frames...");
    let scale = PxScale {
        x: config.font_height,
        y: config.font_height,
    };

    // Precompute text size and position for each lyric
    let mut lyric_positions = Vec::new();
    for lyric in lyrics {
        let (text_width, text_height) = text_size(scale, font, &lyric.text);
        let x = (config.width as i32 - text_width as i32) / 2;
        let y = (config.height as i32 - text_height as i32) / 2;
        lyric_positions.push((lyric, x, y));
    }

    // Create output directory
    std::fs::create_dir_all("output_frames")?;

    for frame in 0..total_frames {
        let time = frame as f32 / config.fps as f32;

        // Create a new image for this frame
        let mut image = RgbImage::new(config.width, config.height);

        // Fill the background with black
        for pixel in image.pixels_mut() {
            *pixel = Rgb([0u8, 0u8, 0u8]);
        }

        // Find the current lyric
        if let Some((lyric, x, y)) = lyric_positions
            .iter()
            .find(|(l, _, _)| time >= l.start_time && time < l.end_time)
        {
            // Draw text
            draw_text_mut(
                &mut image,
                Rgb([255u8, 255u8, 255u8]),
                *x,
                *y,
                scale,
                font,
                &lyric.text,
            );
        }

        // Save the frame
        let frame_filename = format!("output_frames/frame_{:05}.png", frame);
        image.save(&frame_filename)?;
    }

    println!("🖌️ Frames generated successfully!");
    Ok(())
}

fn generate_video(config: &Config, audio_path: &str, total_duration: f32) -> Result<(), Box<dyn Error>> {
    println!("📽️ Generating video using ffmpeg...");
    std::process::Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &config.fps.to_string(),
            "-i",
            "output_frames/frame_%05d.png",
            "-i",
            audio_path,
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-shortest",
            "output_video.mp4",
        ])
        .output()?;
    println!("✅ Video generated successfully!");
    Ok(())
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
