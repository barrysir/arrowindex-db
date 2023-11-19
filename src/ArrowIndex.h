#pragma once

#include <optional>

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

    arrowindex::Difficulty convert_difficulty(Difficulty diff) {
        switch (diff) {
            case Difficulty_Beginner: return arrowindex::Difficulty::Beginner;
            case Difficulty_Easy: return arrowindex::Difficulty::Easy;
            case Difficulty_Medium: return arrowindex::Difficulty::Medium;
            case Difficulty_Hard: return arrowindex::Difficulty::Hard;
            case Difficulty_Challenge: return arrowindex::Difficulty::Challenge;
            case Difficulty_Edit: return arrowindex::Difficulty::Edit;
            default:
                throw std::invalid_argument(RString("Invalid Difficulty enum value given -- ") + DifficultyToString(diff));
        }
    }

    std::optional<arrowindex::Chart> process_chart(const Steps &steps) {
        // ignore charts with an invalid difficulty?
        if (steps.GetDifficulty() == Difficulty_Invalid) {
            return {};
        }

        // Radarvalues are dependent on player because of couples/routine charts
        // For non-couples charts, values are stored in P1 so we'll read from there
        // todo: support couples?
        auto radarvalues = steps.GetRadarValues(PLAYER_1);

        return arrowindex::Chart {
            // idk, maybe can manage something fancier to properly parse stepstype and handle invalid values
            // todo: see how stepmania behaves with invalid steptypes
            .stepstype = steps.m_StepsTypeStr,      // todo
            .difficulty = convert_difficulty(steps.GetDifficulty()),    // todo: unroll this enum
            .description = steps.GetDescription(),
            .meter = steps.GetMeter(),
            .num_steps = (int) radarvalues[RadarCategory_TapsAndHolds],
            .num_mines = (int) radarvalues[RadarCategory_Mines],
            .num_jumps = (int) radarvalues[RadarCategory_Jumps],
            .num_hands = (int) radarvalues[RadarCategory_Hands],
            .num_holds = (int) radarvalues[RadarCategory_Holds],
            .num_rolls = (int) radarvalues[RadarCategory_Rolls],
        };
    }

    arrowindex::Song parse_song(const Song &song) {
		DisplayBpms temp;
		song.GetDisplayBpms(temp);
		float bpm_min = temp.GetMin();
		float bpm_max = temp.GetMax();

        auto song_data = arrowindex::Song {
            // todo - looking for secret songs not supported yet
            .is_secret = false,

            .simfile = song.GetSongFilePath(),

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

            .charts = {},
        };

        for (const Steps* steps : song.GetAllSteps()) {
            if (auto s = process_chart(*steps)) {
                song_data.charts.push_back(*s);
            }
        }

        return song_data;
    }
}