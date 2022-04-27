
mod fingerprinting {
    pub mod communication;
    pub mod algorithm;
    pub mod signature_format;
    mod user_agent;
    mod hanning;
}

mod utils {
    pub mod pulseaudio_loopback;
    pub mod ffmpeg_wrapper;
    pub mod csv_song_history;
    pub mod internationalization;
    pub mod mpris_player;
    pub mod thread;
}

mod core {
    pub mod thread_messages;
}

use crate::fingerprinting::algorithm::SignatureGenerator;
use std::fs;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Audio filename not specified");
    }
    let filename = &args[1];
    println!("{}", filename);
    let signature = SignatureGenerator::make_signature_from_file(filename);
    let bin = signature?.encode_to_binary();
    let outPath = format!("{filename}.shazam");
    fs::write(outPath, bin?).expect("Unable to write file.");
    Ok(())
}