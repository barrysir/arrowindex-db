#pragma once

#include "Song.h"

#include "arrowindex-db/src/lib.rs.h"

namespace arrowindex_utils {
    arrowindex::DisplayBpm convert_bpm(DisplayBPM bpm) {
        switch (bpm) {
            case DISPLAY_BPM_ACTUAL: return arrowindex::DisplayBpm::Actual;
            case DISPLAY_BPM_SPECIFIED: return arrowindex::DisplayBpm::Specified;
            case DISPLAY_BPM_RANDOM: return arrowindex::DisplayBpm::Random;
            default:
                throw std::invalid_argument(RString("Invalid DisplayBPM enum value given -- ") + DisplayBPMToString(bpm));
        }
    }

    arrowindex::Song parse_song(const Song &song) {
		DisplayBpms temp;
		song.GetDisplayBpms(temp);
		float bpm_min = temp.GetMin();
		float bpm_max = temp.GetMax();

        auto song_data = arrowindex::Song {
            // todo - looking for secret songs not supported yet
            .is_secret = false,

            // has_scroll: bool,  // bpm changes, stops, or similar
            // has_mods: bool,  // lua, BGCHANGES, or anything similar
            // has_lua: bool,  // lua scripting specifically
            // has_video: bool,  // does this song show a video
        
            .artist = song.m_sArtist,
            .artisttranslit = song.GetTranslitArtist(),
            .title = song.m_sMainTitle,
            .titletranslit = song.GetTranslitMainTitle(),
            .subtitle = song.m_sSubTitle,
            .subtitletranslit = song.GetTranslitSubTitle(),

            // note: individual charts can override with their own bpm, 
            // but we'll consider this the "main" bpm of the song in the song listing
            .bpmstyle = convert_bpm(song.m_DisplayBPMType), 
            .minbpm = bpm_min,
            .maxbpm = bpm_max,

            .sample_start = song.m_fMusicSampleStartSeconds,
            .sample_length = song.m_fMusicSampleLengthSeconds,

            .length = song.m_fMusicLengthSeconds,      // length of the song as calculated by Stepmania

            .music = song.GetMusicPath(),
            .banner = song.GetBannerPath(),
            .background = song.GetBackgroundPath(),
            .cdtitle = song.GetCDTitlePath(),

            // .charts = Vec<Chart>,
        };

        return song_data;
    }
}