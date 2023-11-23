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

use std::collections::HashMap;

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

    // todo: save this between operations
    let connection = &mut establish_connection();
    let mut model_pack = models::Pack {
        id: None,
        name: &pack.name,
        banner_path: &pack.banner_path,  // todo: copy pack banner to file storage and place new path in here
    };
    
    let pack_id = {
        // look for pack name first, if pack found, use that pack id and update, otherwise insert
        let maybe_existing_pack_id: Option<i32> = (packs::table)
            .filter(packs::name.eq(model_pack.name))   
            .select(packs::id) 
            .first(connection)
            .ok();

        if maybe_existing_pack_id.is_some() {
            model_pack.id = maybe_existing_pack_id;
        }

        let query = diesel::insert_into(packs::table)
            .values(&model_pack)
            .returning(packs::id)
            .on_conflict(packs::id).do_update().set(&model_pack);

        println!("The insert query: {:?}", diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query));
        
        query
            .get_result::<i32>(connection)
            .unwrap()
    };

    // song id of each inserted/updated song, in order of pack.songs

    // insert songs
    let song_ids = {     
        let songs_to_insert = pack.songs.iter().map(|song| {
            use std::path::Path;

            let smpath = Path::new(&song.simfile);
            // to do remove pack name from this
            let songpath = smpath.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
            let smfile = smpath.file_name().unwrap().to_str().unwrap().to_string();

            models::Song {
                id: None,
                pack_id: pack_id,
                song_path: songpath,
                sm_path: smfile,
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
            }
        });

        // fetch songs from the database first
        // generate songs_to_insert
        // compare song paths (bc most similar to how stepmania works)
        //      song titles won't work, because it's easy to have duplicate song titles
        // assign id to matching songs in songs_to_insert
        // partition into {to delete, to insert, to update}
        
        let existing_songs = (songs::table)
            .select((songs::id, songs::song_path))
            .filter(songs::pack_id.eq(pack_id));

        println!("{:?}", diesel::debug_query::<diesel::sqlite::Sqlite, _>(&existing_songs));

        let existing_songs: Vec<(i32, String)> = existing_songs    
            .get_results(connection)
            .unwrap();

        println!("Existing songs {:?}", existing_songs);

        let mut lookup: HashMap<String, i32> = existing_songs.into_iter()
            .map(|(i, s)| (s,i))
            .collect();
        
        println!("Existing song ids {:?}", lookup.values().collect::<Vec<_>>());
        
        let mut to_add: Vec<models::Song> = Vec::new();
        let mut to_update: Vec<models::Song> = Vec::new();
        let mut where_did_song_go: Vec<bool> = Vec::new();
        
        for mut song in songs_to_insert {
            let k = &song.song_path;
            let v = lookup.get(k);
            if v.is_some() {
                song.id = Some(*v.unwrap());
                lookup.remove(k);
                to_update.push(song);
                where_did_song_go.push(false);
            } else {
                to_add.push(song);
                where_did_song_go.push(true);
            }
        }

        println!("Deleting song ids {:?}", lookup.values().collect::<Vec<_>>());
        diesel::delete(songs::table.filter(songs::id.eq_any(lookup.values().collect::<Vec<_>>())))
            .execute(connection)
            .unwrap();

        println!("Updating song ids {:?}", to_update.iter().map(|s| s.id.unwrap()).collect::<Vec<_>>());
        connection.transaction(|connection| {
            for s in &to_update {
                diesel::update(songs::table)
                    .filter(songs::id.eq(s.id.unwrap()))
                    .set(s)
                    .execute(connection)?;
            }

            diesel::result::QueryResult::Ok(())
        }).unwrap();

        let insert_count = diesel::insert_into(songs::table)
            .values(&to_add)
            .execute(connection).unwrap();

        // .get_results() doesn't work with .returning() and batch insert on sqlite 
        // because sqlite doesn't support batch insert with the DEFAULT sql keyword.
        // error[E0271]: type mismatch resolving `<Sqlite as SqlDialect>::InsertWithDefaultKeyword == IsoSqlDefaultKeyword`
        // https://stackoverflow.com/questions/74578751/diesel-get-results-gives-a-trait-bound-error
        // we have to get the song ids another way.
        // I found this approach in the diesel sqlite example code, I'll use it for now:
        // https://github.com/diesel-rs/diesel/blob/2.1.x/examples/sqlite/all_about_inserts/src/lib.rs#L290
        let inserted_song_ids = (songs::table)
            .order(songs::id.desc())
            .limit(insert_count as i64)
            .select(models::SongId::as_select())
            .load(connection)
            .unwrap().into_iter().rev().map(|s| s.id)
            .collect::<Vec<i32>>();

        println!("Inserted {} songs, ids: {:?}", insert_count, inserted_song_ids);

        for (song, inserted_id) in to_add.iter_mut().zip(inserted_song_ids) {
            song.id = Some(inserted_id);
        }

        let mut a = to_update.iter();
        let mut b = to_add.iter();

        let song_ids = where_did_song_go.iter().map(|&v| {
            if v {
                b.next().unwrap().id.unwrap()
            } else {
                a.next().unwrap().id.unwrap()
            }
        });
        song_ids.collect::<Vec<_>>()
    };

    // sanity check: assert chart keys are different for every difficulty in a song
    // how does stepmania manage multiple edits? how does stepmania filter multiple Hard difficulties?
    //if chart.difficulty == "Edit":
    // return (chart.stepstype, chart.difficulty, chart.description)
    // else:
    //     return (chart.stepstype, chart.difficulty)

    let chart_ids = {     
        let charts_to_insert = pack.songs.iter().zip(song_ids.iter()).map(|(song, song_id)| {
            song.charts.iter().map(|chart| {
                models::Chart {
                    id: None,
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
        }).flatten();

        fn hash_chart_model<'a>(chart: &'a models::Chart) -> (i32, String, i32, String) {
            return (chart.song_id, chart.stepstype.clone(), chart.difficulty, chart.description.clone());
        }
        
        let existing_charts = (charts::table)
            .select((charts::id, charts::song_id, charts::stepstype, charts::difficulty, charts::description))
            .filter(charts::song_id.eq_any(&song_ids));

        println!("{:?}", diesel::debug_query::<diesel::sqlite::Sqlite, _>(&existing_charts));

        let existing_charts: Vec<(i32, i32, String, i32, String)> = existing_charts
            .get_results(connection)
            .unwrap();

        println!("Existing charts {:?}", existing_charts);

        let mut lookup: HashMap<_, i32> = existing_charts.into_iter()
            .map(|chart| ((chart.1, chart.2, chart.3, chart.4),chart.0))
            .collect();
        
        println!("Existing charts {:?}", lookup.values().collect::<Vec<_>>());
        
        let mut to_add: Vec<_> = Vec::new();
        let mut to_update: Vec<_> = Vec::new();
        let mut where_did_song_go: Vec<bool> = Vec::new();
        
        for mut chart in charts_to_insert {
            let k = hash_chart_model(&chart);
            let v = lookup.get(&k);
            if v.is_some() {
                chart.id = Some(*v.unwrap());
                lookup.remove(&k);
                to_update.push(chart);
                where_did_song_go.push(false);
            } else {
                to_add.push(chart);
                where_did_song_go.push(true);
            }
        }

        println!("Deleting chart ids {:?}", lookup.values().collect::<Vec<_>>());
        diesel::delete(charts::table.filter(charts::id.eq_any(lookup.values().collect::<Vec<_>>())))
            .execute(connection)
            .unwrap();

        println!("Updating chart ids {:?}", to_update.iter().map(|s| s.id.unwrap()).collect::<Vec<_>>());
        connection.transaction(|connection| {
            for s in &to_update {
                diesel::update(charts::table)
                    .filter(charts::id.eq(s.id.unwrap()))
                    .set(s)
                    .execute(connection)?;
            }

            diesel::result::QueryResult::Ok(())
        }).unwrap();

        let insert_count = diesel::insert_into(charts::table)
            .values(&to_add)
            .execute(connection).unwrap();

        // .get_results() doesn't work with .returning() and batch insert on sqlite 
        // because sqlite doesn't support batch insert with the DEFAULT sql keyword.
        // error[E0271]: type mismatch resolving `<Sqlite as SqlDialect>::InsertWithDefaultKeyword == IsoSqlDefaultKeyword`
        // https://stackoverflow.com/questions/74578751/diesel-get-results-gives-a-trait-bound-error
        // we have to get the song ids another way.
        // I found this approach in the diesel sqlite example code, I'll use it for now:
        // https://github.com/diesel-rs/diesel/blob/2.1.x/examples/sqlite/all_about_inserts/src/lib.rs#L290
        let inserted_chart_ids = (charts::table)
            .order(charts::id.desc())
            .limit(insert_count as i64)
            .select(charts::id)
            .load(connection)
            .unwrap().into_iter().rev()
            .collect::<Vec<i32>>();

        println!("Inserted {} songs, ids: {:?}", insert_count, inserted_chart_ids);

        for (chart, inserted_id) in to_add.iter_mut().zip(inserted_chart_ids) {
            chart.id = Some(inserted_id);
        }

        let mut a = to_update.iter();
        let mut b = to_add.iter();

        let chart_ids = where_did_song_go.iter().map(|&v| {
            if v {
                b.next().unwrap().id.unwrap()
            } else {
                a.next().unwrap().id.unwrap()
            }
        });
        chart_ids.collect::<Vec<_>>()
    };
}

pub fn process_new_pack(pack: ffi::Pack) -> () {
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