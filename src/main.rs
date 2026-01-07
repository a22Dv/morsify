/*
    main.rs
    Application entry point.
 */

mod morsify;

use std::env;
use rodio::{OutputStreamBuilder, Sink};

fn main() {
    let argv = env::args().collect::<Vec<String>>(); 
    if argv.len() != 3 {
        println!("Usage: morsify -t/--translate/-p/--playback <TEXT>");
        return;
    }

    let flag = argv[1].as_str();
    let argument = &argv[2];

    print!("\x1B[2J\x1b[H");
    let text = morsify::morsify_text(argument).expect("Invalid character detected... Exiting...");
    match flag {
        "-p" | "--playback" => {
            let strm_hndlr = OutputStreamBuilder::open_default_stream();
            let mut stream_handle = strm_hndlr.expect("Could not acquire a handle to output audio device... Exiting...");
            let mut sink = Sink::connect_new(&stream_handle.mixer());
            morsify::morsify_playback(&argument, &text, &mut sink).expect("Encountered a UTF-8 encoding error... Exiting...");
            sink.sleep_until_end();
            stream_handle.log_on_drop(false);
        }  
        "-t" | "--translate" => {
            println!("TEXT: {}\nMORSE CODE: {}", argument, text);
        }
        _ => {
            println!("Unknown option... Exiting...");
            println!("Usage: morsify -t/--translate/-p/--playback <TEXT>");
        }
    }
}