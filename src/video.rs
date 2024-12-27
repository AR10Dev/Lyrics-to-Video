use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use std::error::Error;
use std::fs;

pub fn create_output_directory() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("output_frames")?;
    Ok(())
}

pub fn generate_frames(
    width: u32,
    height: u32,
    fps: u32,
    font_height: f32,
    font: &ab_glyph::FontVec,
    lyrics: &[crate::lyrics::Lyric],
    total_frames: u32,
) -> Result<(), Box<dyn Error>> {
    println!("🎨 Generating frames...");
    let scale = ab_glyph::PxScale {
        x: font_height,
        y: font_height,
    };

    // Precompute text size and position for each lyric
    let mut lyric_positions = Vec::new();
    for lyric in lyrics {
        let (text_width, text_height) = text_size(scale, font, &lyric.text);
        let x = (width as i32 - text_width as i32) / 2;
        let y = (height as i32 - text_height as i32) / 2;
        lyric_positions.push((lyric, x, y));
    }

    // Create output directory
    create_output_directory()?;

    for frame in 0..total_frames {
        let time = frame as f32 / fps as f32;

        // Create a new image for this frame
        let mut image = RgbImage::new(width, height);

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

pub fn generate_video(
    fps: u32,
    audio_path: &str,
    total_duration: f32,
) -> Result<(), Box<dyn Error>> {
    println!("📽️ Generating video using ffmpeg...");
    std::process::Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &fps.to_string(),
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
