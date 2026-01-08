/*
    morsify.rs
    Application core logic.
 */


use std::io;
use std::thread;
use std::io::Write;
use std::str::Utf8Error;
use std::time::Duration;
use rodio::{Sink, Source};

const MAX_CODE_LENGTH: usize = 5;
const ASCII_RANGE: usize = 128;

const INTERVAL_BASE: Duration = Duration::from_millis(50);
const INTERVAL_WORD_MS: Duration = Duration::from_millis((INTERVAL_BASE.as_millis() * 7) as u64);
const INTERVAL_LETTER_MS: Duration = Duration::from_millis((INTERVAL_BASE.as_millis() * 3) as u64);
const TRANSLATION_MAP: [&str; ASCII_RANGE] = initialize_translation_map();

const fn initialize_translation_map() -> [&'static str; 128] {
    let mut translation_map = [""; 128];

    const MORSE_CODE: [&str; 36] = [ // Coded from A-Z, 0->9, in that order.
        ".-", "-...", "-.-.", "-..", ".", "..-.", 
        "--.", "....", "..", ".---", "-.-", ".-..", 
        "--", "-.", "---", ".--.", "--.-", ".-.", 
        "...", "-", "..-", "...-", ".--", "-..-", 
        "-.--", "--..", "-----", ".----", "..---", "...--", 
        "....-", ".....", "-....", "--...", 
        "---..", "----."
    ];

    let mut it: usize = 0;
    while it < 26 {
        translation_map[b'A' as usize + it] = MORSE_CODE[it];
        translation_map[b'a' as usize + it] = MORSE_CODE[it];
        it += 1;
    }
    while it < 26 + 10 {
        translation_map[b'0' as usize + (it - 26)] = MORSE_CODE[it];
        it += 1;
    }
    translation_map[b' ' as usize] = "|"; // Special-case handling.
    translation_map
}

struct Signal {
    phase: f32,
    tsample: usize,
    sample_count: usize,
    duration: Duration
}

impl Signal {
    fn new(duration: Duration) -> Signal {
        Signal{
            phase: 0.0, 
            tsample: 0, 
            sample_count: (duration.as_secs_f32() * 48000.0) as usize, 
            duration
        }
    }
}

impl Iterator for Signal {
    type Item = f32;
    fn next(self: &mut Self) -> Option<Self::Item> {
        const SAMPLE_THRESHOLD: usize = 250;
        if self.sample_count - self.tsample <= 0 {
            return None;
        }
        self.phase += 2.0 * 3.1415927 * 450.0 / 48000.0;
        self.phase %= 2.0 * 3.1415927;

        let mut sample = self.phase.sin();
        if self.sample_count < SAMPLE_THRESHOLD {
            sample *= self.sample_count as f32 / SAMPLE_THRESHOLD as f32;
        } else if self.sample_count - self.tsample < SAMPLE_THRESHOLD {
            sample *= (self.sample_count - self.tsample) as f32 / SAMPLE_THRESHOLD as f32;
        }
        self.tsample += 1;
        Some(sample)
    }
}

impl Source for Signal {
    fn current_span_len(&self) -> Option<usize> { Some((self.sample_count - self.tsample).max(0)) }
    fn channels(&self) -> rodio::ChannelCount { 1 }
    fn sample_rate(&self) -> rodio::SampleRate { 48000 }
    fn total_duration(&self) -> Option<Duration> { Some(self.duration) }
}

pub fn morsify_text(text: &String) -> Result<String, ()> {
    let mut morsified = String::new();
    for ch in text.as_bytes() {
        if ch.is_ascii_alphanumeric() || *ch == b' '{
            morsified.push_str(TRANSLATION_MAP[*ch as usize]);
            morsified.push_str(" ");
            continue;
        }
        return Err(());   
    }
    Ok(morsified)
}

pub fn morsify_playback(input: &String, code: &String, sink: &mut Sink) -> Result<(), Utf8Error> {
    let mut it = 0;
    let code_bytes = code.as_bytes();
    let mut stdout = io::stdout();

    println!("--- OUTPUT START ---");
    for ch in input.as_bytes() {
        match ch {
            b' ' => {
                it += 1;
                println!(""); // Space out formatting.
                thread::sleep(INTERVAL_WORD_MS);
            }, 
            _ => {
                let mut characters = [b' '; MAX_CODE_LENGTH];
                let mut ch_it = 0;
                while code_bytes[it] != b' ' {
                    characters[ch_it] = code_bytes[it];
                    ch_it += 1;
                    it += 1;
                }
                println!("\"{}\": {}", ch.to_ascii_uppercase() as char, str::from_utf8(&characters)?);
                let _ = stdout.flush();
                morsify_sound(&characters, sink);
                thread::sleep(INTERVAL_LETTER_MS);
            }
        }
        it += 1;
    }
    println!("--- OUTPUT END ---");
    Ok(())
}

fn morsify_sound(code: &[u8; MAX_CODE_LENGTH], sink: &mut Sink) {
    const DOT_DURATION: Duration = INTERVAL_BASE;
    const DASH_DURATION: Duration = Duration::from_millis((INTERVAL_BASE.as_millis() * 3) as u64);
    for cd in *code {
        if cd == b' ' {
            break;
        }
        if cd == b'.' {
            sink.append(Signal::new(DOT_DURATION));
        } else { // cd == b'-'
            sink.append(Signal::new(DASH_DURATION));
        }
        sink.sleep_until_end();
        thread::sleep(INTERVAL_BASE);
    }
}