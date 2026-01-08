# morsify
![LANGUAGE](https://img.shields.io/badge/Rust-red?logo=rust&logoColor=white)
![MIT License](https://img.shields.io/badge/License-MIT-green)

A minimal morse code audio synthesizer in Rust.

Serves as a personal introduction into DSP (Digital Signal Processing),
as well as audio-handling in Rust using the `rodio` crate.

## Getting Started

Example command:

```
git clone https://github.com/a22Dv/morsify.git 
cargo build && cargo run -- -p "Hello World"
```
>[!NOTE]
>You can change `-p` to `-t` if all you need is the morse code translation
>and not the audio itself. 

>[!NOTE]
>This program does not handle special characters, only alphanumerics and
>the space character.

## License

This project is licensed under the MIT License - see LICENSE for more details.

## Author

a22Dv - a22dev.gl@gmail.com