use diesel::prelude::*;

#[derive(Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::packs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]       // todo: remove this to be more SQL platform agnostic
pub struct Pack<'a> {
    pub id: Option<i32>,
    pub name: &'a String,
    pub banner_path: &'a String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]       // todo: remove this to be more SQL platform agnostic
pub struct SongId {
    pub id: i32,
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]       // todo: remove this to be more SQL platform agnostic
pub struct Song<'a> {
    pub id: Option<i32>,
    pub pack_id: i32,
    pub song_path: String,
    pub sm_path: String,
    pub artist: &'a String,
    pub artisttranslit: &'a String,
    pub title: &'a String,
    pub titletranslit: &'a String,
    pub subtitle: &'a String,
    pub subtitletranslit: &'a String,
    pub bpmstyle: i32,
    pub minbpm: f32,
    pub maxbpm: f32,
    pub length: f32,
    pub sample_start: f32,
    pub sample_length: f32,
    pub banner_path: &'a String,
    pub background_path: &'a String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::charts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]       // todo: remove this to be more SQL platform agnostic
pub struct Chart<'a> {
    // pub id: i32,
    pub song_id: i32,
    pub stepstype: &'a String,
    pub difficulty: i32,
    pub description: &'a String,
    pub meter: i32,
    pub num_steps: i32,
    pub num_mines: i32,
    pub num_jumps: i32,
    pub num_hands: i32,
    pub num_holds: i32,
    pub num_rolls: i32,
}

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