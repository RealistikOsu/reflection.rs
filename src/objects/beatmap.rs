use crate::consts::Status;

/// # osu! Beatmap
/// This structure represents a singular beatmap (single difficulty) internally.
/// This data may be used in responses.
struct Beatmap {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub difficulty: String,
    pub status: Status,
    pub mode: u8,
    pub max_combo: u16,
    pub hit_length: u16,
    pub total_length: u16,
    // Some bancho stats (mb rosu later)
    pub playcount: u32,
    pub passcount: u32,
    // Star rating will be weird.
    pub sr_std: Option<f32>,
    pub sr_taiko: Option<f32>,
    pub sr_ctb: Option<f32>,
    pub sr_mania: Option<f32>,
    // Bmap diff stuff
    pub ar: f32,
    pub bpm: f32,
    pub cs: f32,
    pub hp: f32
}

/// # osu! Beatmap Set
/// A struct representing a collection (set) of Beatmaps under the same song.
struct BeatmapSet {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub mapper: String,
    pub children: Vec<Beatmap>,
    pub tags: String,
    pub source: String,
    pub genre: u8,
    pub video: bool
}