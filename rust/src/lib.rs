pub mod models;
pub mod schema;

#[cxx::bridge(namespace = "arrowindex")]
mod ffi {
    struct Pack {
        name: String,
        banner_path: String,    // path to banner relative to itgmania
        songs: Vec<Song>,
    }

    struct Song {
        is_secret: bool,    // is a "secret" song (valid simfile hidden in the pack files)

        // has_scroll: bool,  // bpm changes, stops, or similar
        // has_mods: bool,  // lua, BGCHANGES, or anything similar
        // has_lua: bool,  // lua scripting specifically
        // has_video: bool,  // does this song show a video

        // path: String,           // Path to song directory relative to itgmania (is this needed?)
        simfile: String,        // Path to the parsed simfile relative to itgmania
    
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
    
        music: String,        // Path to the audio file relative to itgmania
        banner: String,       // Path to the banner relative to itgmania
        background: String,   // Path to the background relative to itgmania
        cdtitle: String,      // Path to the cd title relative to itgmania
    
        charts: Vec<Chart>,
    
        // speedchanges: str,    // bpm and speed changes, stored in JSON, to be rendered as a graph on the client
    }

    struct Chart {
        stepstype: String,          // can deduplicate these into an enum or table if i want
        difficulty: Difficulty,     // can deduplicate these into an enum or table if i want
        description: String,
        meter: i32,
        // radarvalues: String,        // todo: parse this?
    
        // hash: String,      // can store as integer to make smaller
    
        // don't bother storing raw difficulties, only what Stepmania parses
        // difficulty_raw: String,
    
        num_steps: u32,
        num_mines: u32,
        num_jumps: u32,
        num_hands: u32,
        num_holds: u32,
        num_rolls: u32,
        
        // song_overrides: String, // if this chart has its own bpm changes or other unique values, put it here. JSON format
    }

    enum DisplayBpm {
        Actual,
        Specified,
        Random,
    }

    #[derive(Hash)]
    enum Difficulty {
        Beginner,
        Easy,
        Medium,
        Hard,
        Challenge,
        Edit,
        Invalid,
    }

    extern "Rust" {
        fn rust_from_cpp() -> ();
        fn process_new_pack(pack: Pack) -> ();
    }
}

pub fn rust_from_cpp() -> () {
    println!("called rust_from_cpp()");
}

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
// use dotenvy::dotenv;
// use std::env;

pub fn establish_connection() -> SqliteConnection {
    // dotenv().ok();

    let database_url = "sqlite://arrowindex.db"; // env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_pack(pack: ffi::Pack) -> () {    
    // dev note: this function takes ownership of the pack struct so it can 
    // move data (strings, etc.) into the models::Pack object without cloning

    use self::schema::packs::dsl::*;

    let connection = &mut establish_connection();
    let model_pack = models::Pack {
        name: pack.name,
        banner_path: pack.banner_path,  // todo: copy pack banner to file storage and place new path in here
    };
    diesel::insert_into(packs).values(&model_pack).execute(connection).unwrap();
    // diesel::update(packs::table).set(model_pack).execute(connection)
    // let results = packs
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .select(Post::as_select())
    //     .load(connection)
    //     .expect("Error loading posts");
}

pub fn process_new_pack(pack: ffi::Pack) -> () {
    use std::collections::HashMap;
    use ffi::Difficulty;
    use ffi::Chart;

    println!("Rust got a new pack: {} ({} songs)", pack.name, pack.songs.len());

    for song in pack.songs.iter() {
        let bpmstring = {
            if song.minbpm == song.maxbpm { format!("{}", song.minbpm) }
            else { format!("{}-{}", song.minbpm, song.maxbpm) }
        };
        
        let diffspread = {
            let lookup = song.charts.iter().filter(|c| c.stepstype == "dance-single").map(|c| (c.difficulty, c)).collect::<HashMap<Difficulty, &Chart>>();
            let spread = [Difficulty::Beginner, Difficulty::Easy, Difficulty::Medium, Difficulty::Hard, Difficulty::Challenge];
            format!("| {} |", spread.map(|d| lookup.get(&d).map(|c| c.meter.to_string()).unwrap_or("-".to_string())).join(" | "))
        };

        println!("\t{} {} - {} ({} BPM, {} seconds) simfile:{}", diffspread, song.artist, song.title, bpmstring, song.length, song.simfile);
    }

    insert_pack(pack);
}