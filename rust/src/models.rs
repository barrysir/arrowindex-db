use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::packs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]       // todo: remove this to be more SQL platform agnostic
pub struct Pack {
    pub name: String,
    pub banner_path: String,
}

// struct Song {
//     is_secret: bool,    // is a "secret" song (valid simfile hidden in the pack files)

//     // has_scroll: bool,  // bpm changes, stops, or similar
//     // has_mods: bool,  // lua, BGCHANGES, or anything similar
//     // has_lua: bool,  // lua scripting specifically
//     // has_video: bool,  // does this song show a video

//     path: String,       // absolute path to song directory

//     artist: String,
//     artisttranslit: String,
//     title: String,
//     titletranslit: String,
//     subtitle: String,
//     subtitletranslit: String,

//     // individual charts can have their own bpm, but we'll consider this the "main" bpm of the song
//     // in the song listing
//     bpmstyle: DisplayBpm, 
//     minbpm: f32,
//     maxbpm: f32,

//     sample_start: f32,
//     sample_length: f32,

//     length: f32,      // length of the song as calculated by Stepmania

//     music: String,        // Path to the audio file relative to song directory
//     banner: String,       // Path to the banner relative to song directory
//     background: String,   // Path to the background relative to song directory
//     cdtitle: String,      // Path to the cd title relative to song directory

//     simfile: String,      // Path to the parsed simfile relative to song directory

//     charts: Vec<Chart>,

//     // speedchanges: str,    // bpm and speed changes, stored in JSON, to be rendered as a graph on the client
// }

// struct Chart {
//     stepstype: String,          // can deduplicate these into an enum or table if i want
//     difficulty: Difficulty,     // can deduplicate these into an enum or table if i want
//     description: String,
//     meter: i32,
//     // radarvalues: String,        // todo: parse this?

//     // hash: String,      // can store as integer to make smaller

//     // don't bother storing raw difficulties, only what Stepmania parses
//     // difficulty_raw: String,

//     num_steps: u32,
//     num_mines: u32,
//     num_jumps: u32,
//     num_hands: u32,
//     num_holds: u32,
//     num_rolls: u32,
    
//     // song_overrides: String, // if this chart has its own bpm changes or other unique values, put it here. JSON format
// }