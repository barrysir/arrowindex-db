// @generated automatically by Diesel CLI.

diesel::table! {
    charts (id) {
        id -> Integer,
        song_id -> Integer,
        stepstype -> Text,
        difficulty -> Integer,
        description -> Text,
        meter -> Integer,
        num_steps -> Integer,
        num_mines -> Integer,
        num_jumps -> Integer,
        num_hands -> Integer,
        num_holds -> Integer,
        num_rolls -> Integer,
    }
}

diesel::table! {
    packs (id) {
        id -> Integer,
        name -> Text,
        banner_path -> Text,
    }
}

diesel::table! {
    songs (id) {
        id -> Integer,
        pack_id -> Integer,
        artist -> Text,
        artisttranslit -> Text,
        title -> Text,
        titletranslit -> Text,
        subtitle -> Text,
        subtitletranslit -> Text,
        bpmstyle -> Integer,
        minbpm -> Float,
        maxbpm -> Float,
        length -> Float,
        sample_start -> Float,
        sample_length -> Float,
        banner_path -> Text,
        background_path -> Text,
        sm_path -> Text,
    }
}

diesel::joinable!(charts -> songs (song_id));
diesel::joinable!(songs -> packs (pack_id));

diesel::allow_tables_to_appear_in_same_query!(
    charts,
    packs,
    songs,
);
