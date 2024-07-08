use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rodio::{Decoder, Source};
use std::error::Error;
use std::fs::File;

struct Lyric {
    text: String,
    start_time: f32,
    end_time: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load font
    println!("🔍 Loading font...");
    let font = FontRef::try_from_slice(include_bytes!("../font.ttf"))?;

    // Load audio file
    println!("🎵 Loading audio file...");
    let audio_file = File::open("../audio.mp3")?;

    println!("🎬 Generating video...");
    let source: Decoder<File> = Decoder::new(audio_file)?;
    let total_duration = source.total_duration().unwrap().as_secs_f32();

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

    // Video settings
    let width = 1920;
    let height = 1080;
    let fps = 30;
    let total_frames = (total_duration * fps as f32) as u32;

    println!("🖼️ Total duration: {} seconds", total_duration);
    println!("🎞️ Total frames: {}", total_frames);

    // Text settings
    let font_height = 60.0;
    let scale = PxScale {
        x: font_height,
        y: font_height,
    };

    // Create output directory
    std::fs::create_dir_all("output_frames")?;

    // Generate frames
    println!("🎨 Generating frames...");
    for frame in 0..total_frames {
        let time = frame as f32 / fps as f32;

        // Create a new image for this frame
        let mut image = RgbImage::new(width, height);

        // Fill the background with black
        for pixel in image.pixels_mut() {
            *pixel = Rgb([0u8, 0u8, 0u8]);
        }

        // Find the current lyric
        if let Some(lyric) = lyrics
            .iter()
            .find(|l| time >= l.start_time && time < l.end_time)
        {
            // Calculate text size and position
            let (text_width, text_height) = text_size(scale, &font, &lyric.text);
            let x = (width as i32 - text_width as i32) / 2;
            let y = (height as i32 - text_height as i32) / 2;

            // Draw text
            draw_text_mut(
                &mut image,
                Rgb([255u8, 255u8, 255u8]),
                x,
                y,
                scale,
                &font,
                &lyric.text,
            );
        }

        // Save the frame
        let frame_filename = format!("output_frames/frame_{:05}.png", frame);
        image.save(&frame_filename)?;
    }

    println!("🖌️ Frames generated successfully!");

    // Use ffmpeg to combine frames and audio (you would need to install ffmpeg separately)
    println!("📽️ Generating video using ffmpeg...");
    std::process::Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &fps.to_string(),
            "-i",
            "output_frames/frame_%05d.png",
            "-i",
            "../audio.mp3",
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
