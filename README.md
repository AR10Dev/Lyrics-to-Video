# Rust Lyric Video Generator

This project is a simple lyric video generator written in Rust. It takes an audio file and a set of lyrics, and generates a video with the lyrics displayed in sync with the audio.

## 🚀 Features

- Generate lyric videos from audio files and text
- Customizable video resolution and frame rate
- Simple black background with white text
- Uses ffmpeg for final video compilation

## 📋 Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- ffmpeg installed and available in your system PATH
- A TrueType font file (.ttf) for rendering text

## 🛠️ Installation

1. Clone this repository:
   ```
   git clone https://github.com/AR10Dev/Lyrics-to-Video.git
   cd Lyrics-to-Video
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## 📝 Usage

1. Place your audio file (MP3 format) in the project root and name it `audio.mp3`.
2. Place your font file (TTF format) in the project root and name it `font.ttf`.
3. Edit the `lyrics` vector in `src/lyrics.rs` to include your lyrics and their timings.
4. Run the program:
   ```
   cargo run --release
   ```

The program will generate a series of PNG frames in the `output_frames` directory and then use ffmpeg to combine these frames with the audio into a final MP4 video named `output_video.mp4`.

## ⚙️ Configuration

You can modify the following settings in `src/main.rs`:

- Video resolution: Adjust `width` and `height` fields in the `Config` struct
- Frame rate: Modify the `fps` field in the `Config` struct
- Font size: Change the `font_height` field in the `Config` struct

## 📦 Dependencies

This project uses the following Rust crates:

- `ab_glyph` for font handling
- `image` for image creation and manipulation
- `imageproc` for drawing text on images
- `rodio` for audio file parsing

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check [issues page](https://github.com/AR10Dev/Lyrics-to-Video/issues).

## 📜 License

This project is [GPL-3.0](https://choosealicense.com/licenses/gpl-3.0/) licensed.

## 🙏 Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [ffmpeg](https://ffmpeg.org/)
- All the awesome Rust crate developers
