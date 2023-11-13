#[cxx::bridge(namespace = "arrowindex")]
mod ffi {
    struct Pack {
        name: String,
        banner_path: String,
        songs: Vec<Song>,
    }

    struct Song {
        is_secret: bool,    // is a "secret" song (valid simfile hidden in the pack files)

        // has_scroll: bool,  // bpm changes, stops, or similar
        // has_mods: bool,  // lua, BGCHANGES, or anything similar
        // has_lua: bool,  // lua scripting specifically
        // has_video: bool,  // does this song show a video
    
        artist: String,
        artisttranslit: String,
        title: String,
        titletranslit: String,
        subtitle: String,
        subtitletranslit: String,
    
        // individual charts can have their own bpm, but we'll consider this the "main" bpm of the song
        // in the song listing
        bpmstyle: DisplayBpm, 
        minbpm: f32,
        maxbpm: f32,

        sample_start: f32,
        sample_length: f32,
    
        length: f32,      // length of the song as calculated by Stepmania
    
        music: String,        // SM ONLY. Path to the audio file
        banner: String,       // SM ONLY. Path to the banner
        background: String,   // SM ONLY. Path to the background
        cdtitle: String,      // SM ONLY. Path to the cd title
    
        // charts: Vec<Chart>,
    
        // speedchanges: str,    // bpm and speed changes, stored in JSON, to be rendered as a graph on the client
    }

    enum DisplayBpm {
        Actual,
        Specified,
        Random,
    }

    extern "Rust" {
        fn rust_from_cpp() -> ();
        fn process_new_pack(pack: Pack) -> ();
    }
}

pub fn rust_from_cpp() -> () {
    println!("called rust_from_cpp()");
}

pub fn process_new_pack(pack: ffi::Pack) -> () {
    println!("Rust got a new pack: {} ({} songs)", pack.name, pack.songs.len());

    for song in pack.songs.iter() {
        let bpmstring = {
            if song.minbpm == song.maxbpm { format!("{}", song.minbpm) }
            else { format!("{}-{}", song.minbpm, song.maxbpm) }
        };
        println!("\t{} - {} ({} BPM, {} seconds)", song.artist, song.title, bpmstring, song.length);
    }
}