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
3. Create a `config.json` file in the project root with the following structure:
   ```json
   {
     "width": 1920,
     "height": 1080,
     "fps": 30,
     "font_height": 60.0,
     "audio_path": "audio.mp3",
     "font_path": "font.ttf",
     "output_directory": "output_frames",
     "lyrics": [
       {
         "text": "First line of lyrics",
         "start_time": 0.0,
         "end_time": 2.0
       },
       {
         "text": "Second line of lyrics",
         "start_time": 2.0,
         "end_time": 4.0
       }
       // Add more lyrics...
     ]
   }
   ```
4. Run the program:
   ```
   cargo run --release
   ```

The program will generate a series of PNG frames in the `output_frames` directory and then use ffmpeg to combine these frames with the audio into a final MP4 video named `output_video.mp4`.

## ⚙️ Configuration

You can modify the following settings in `config.json`:

- Video resolution: Adjust `width` and `height` fields
- Frame rate: Modify the `fps` field
- Font size: Change the `font_height` field
- Audio file path: Update the `audio_path` field
- Font file path: Update the `font_path` field
- Output directory: Change the `output_directory` field
- Lyrics: Add or modify the lyrics and their timings in the `lyrics` array

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
