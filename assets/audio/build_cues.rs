// Build script to generate WAV files from synthesizer.
// Run with: rustc synthesizer.rs build_cues.rs -o build_cues && ./build_cues

mod synthesizer {
    include!("synthesizer.rs");
}

use std::fs;
use std::path::Path;

fn main() {
    let cue_types = vec![
        "rule-fire",
        "achievement",
        "intervention-warn",
        "focus-start",
        "focus-end",
        "error",
        "success",
        "mascot-acknowledge",
    ];

    let cues_dir = Path::new("cues");
    fs::create_dir_all(cues_dir).expect("Failed to create cues directory");

    for cue in cue_types {
        let wav = synthesizer::generate_wav(cue);
        let filename = cues_dir.join(format!("{}.wav", cue));
        fs::write(&filename, wav).expect(&format!("Failed to write {}", filename.display()));
        println!("Generated: {}", filename.display());
    }
}
