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
    
        num_steps: i32,
        num_mines: i32,
        num_jumps: i32,
        num_hands: i32,
        num_holds: i32,
        num_rolls: i32,
        
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

    // todo: don't hardcode this
    let database_url = "sqlite://arrowindex.db"; // env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_pack(pack: &ffi::Pack) -> () {
    use self::schema::packs;
    use self::schema::songs;
    use self::schema::charts;

    // insert pack record
    let connection = &mut establish_connection();
    let model_pack = models::Pack {
        name: &pack.name,
        banner_path: &pack.banner_path,  // todo: copy pack banner to file storage and place new path in here
    };
    
    let pack_id = {
        let pack_id = diesel::insert_into(packs::table)
            .values(&model_pack)
            .returning(packs::id)
            .get_result::<i32>(connection)
            .unwrap();
        pack_id
    };

    // insert songs
    let songs_to_insert = pack.songs.iter().map(|song| {
        models::Song {
            pack_id: pack_id,
            artist: &song.artist,
            artisttranslit: &song.artisttranslit,
            title: &song.title,
            titletranslit: &song.titletranslit,
            subtitle: &song.subtitle,
            subtitletranslit: &song.subtitletranslit,
            bpmstyle: song.bpmstyle.repr as i32,
            minbpm: song.minbpm,
            maxbpm: song.maxbpm,
            length: song.length,
            sample_start: song.sample_start,
            sample_length: song.sample_length,
            banner_path: &song.banner,
            background_path: &song.background,
            sm_path: &song.simfile, 
        }
    }).collect::<Vec<models::Song>>();

    let song_ids = {
        let insert_count = diesel::insert_into(songs::table)
            .values(&songs_to_insert)
            .execute(connection).unwrap();

        // let song_ids = diesel::insert_into(songs::table)
        //     .values(&songs_to_insert)
        //     .returning(songs::id)
        //     .get_results(connection)
        //     .unwrap();

        // .get_results() doesn't work with .returning() and batch insert on sqlite 
        // because sqlite doesn't support batch insert with the DEFAULT sql keyword.
        // error[E0271]: type mismatch resolving `<Sqlite as SqlDialect>::InsertWithDefaultKeyword == IsoSqlDefaultKeyword`
        // https://stackoverflow.com/questions/74578751/diesel-get-results-gives-a-trait-bound-error
        // we have to get the song ids another way.
        // I found this approach in the diesel sqlite example code, I'll use it for now:
        // https://github.com/diesel-rs/diesel/blob/2.1.x/examples/sqlite/all_about_inserts/src/lib.rs#L290
        (songs::table)
            .order(songs::id.desc())
            .limit(insert_count as i64)
            .select(models::SongId::as_select())
            .load(connection)
            .unwrap().into_iter().rev().map(|s| s.id)
            .collect::<Vec<i32>>()
    };

    // insert charts
    let charts_to_insert = pack.songs.iter().zip(song_ids.iter()).map(|(song, song_id)| {
        song.charts.iter().map(|chart| {
            models::Chart {
                song_id: *song_id,
                stepstype: &chart.stepstype,
                difficulty: chart.difficulty.repr as i32,
                description: &chart.description,
                meter: chart.meter,
                num_steps: chart.num_steps,
                num_mines: chart.num_mines,
                num_jumps: chart.num_jumps,
                num_hands: chart.num_hands,
                num_holds: chart.num_holds,
                num_rolls: chart.num_rolls,
            }
        })
    }).flatten().collect::<Vec<models::Chart>>();

    diesel::insert_into(charts::table)
        .values(&charts_to_insert)
        .execute(connection)
        .unwrap();
    
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

    insert_pack(&pack);
}