CREATE TABLE packs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,       -- SERIAL type not supported in sqlite, use INTEGER AUTOINCREMENT NOT NULL
    name VARCHAR NOT NULL,
    banner_path VARCHAR NOT NULL
);

CREATE TABLE songs (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    pack_id INTEGER NOT NULL,

    song_path VARCHAR NOT NULL,     -- Path to the song folder relative to the pack
    sm_path VARCHAR NOT NULL,       -- Path to the sm file within the song folder

    artist VARCHAR NOT NULL,
    artisttranslit VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    titletranslit VARCHAR NOT NULL,
    subtitle VARCHAR NOT NULL,
    subtitletranslit VARCHAR NOT NULL,

    bpmstyle INTEGER NOT NULL,      -- enum
    minbpm REAL NOT NULL,
    maxbpm REAL NOT NULL,

    length REAL NOT NULL,

    sample_start REAL NOT NULL,
    sample_length REAL NOT NULL,

    banner_path VARCHAR NOT NULL,       -- Path to the banner image in file storage
    background_path VARCHAR NOT NULL,   -- Path to the background image in file storage

    FOREIGN KEY (pack_id) REFERENCES packs (id)
);

CREATE TABLE charts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,

    song_id INTEGER NOT NULL,

    stepstype VARCHAR NOT NULL,
    difficulty INTEGER NOT NULL,        -- enum
    description VARCHAR NOT NULL,
    meter INTEGER NOT NULL,

    num_steps INTEGER NOT NULL,
    num_mines INTEGER NOT NULL,
    num_jumps INTEGER NOT NULL,
    num_hands INTEGER NOT NULL,
    num_holds INTEGER NOT NULL,
    num_rolls INTEGER NOT NULL,
    
    FOREIGN KEY (song_id) REFERENCES songs (id)
);