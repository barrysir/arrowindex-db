// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Int4,
        song_id -> Int4,
        stepstype -> Varchar,
        difficulty -> Int4,
        description -> Varchar,
        meter -> Int4,
        num_steps -> Int4,
        num_mines -> Int4,
        num_jumps -> Int4,
        num_hands -> Int4,
        num_holds -> Int4,
        num_rolls -> Int4,
    }
}

diesel::table! {
    packs (id) {
        id -> Int4,
        name -> Varchar,
        banner_path -> Varchar,
    }
}

diesel::table! {
    songs (id) {
        id -> Int4,
        pack_id -> Int4,
        song_path -> Varchar,
        sm_path -> Varchar,
        artist -> Varchar,
        artisttranslit -> Varchar,
        title -> Varchar,
        titletranslit -> Varchar,
        subtitle -> Varchar,
        subtitletranslit -> Varchar,
        bpmstyle -> Int4,
        minbpm -> Float4,
        maxbpm -> Float4,
        length -> Float4,
        sample_start -> Float4,
        sample_length -> Float4,
        banner_path -> Varchar,
        background_path -> Varchar,
        tags -> Varchar,
    }
}

diesel::joinable!(charts -> songs (song_id));
diesel::joinable!(songs -> packs (pack_id));

diesel::allow_tables_to_appear_in_same_query!(
    charts,
    packs,
    songs,
);
