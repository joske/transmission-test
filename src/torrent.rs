use bip_metainfo::{MetainfoBuilder, PieceLength};

#[allow(dead_code)]
/// Create a torrent file from a folder.
pub fn create_torrent_file(folder: &str, output_file: &str) {
    let builder = MetainfoBuilder::new()
        .set_piece_length(PieceLength::OptBalanced)
        .set_main_tracker(Some("udp://tracker.opentrackr.org:1337/announce"));
    let bytes = builder.build(1, folder, |_| {}).unwrap();
    std::fs::write(output_file, bytes).unwrap();
}

#[allow(dead_code)]
pub trait BitTorrent {
    /// Add a torrent file to Transmission. The torrents starts downloading/seeding immediately.
    async fn add(&self, torrent_file: &str) -> Torrent;
    /// Stop torrents by their IDs. The IDs should be the torrent hash.
    async fn stop(&self, ids: Vec<String>);
    /// List all torrents.
    async fn list(&self) -> Vec<Torrent>;
    /// Remove torrents by their IDs (torrent hash). If `delete_local_data` is true, the local data will also be deleted.
    async fn remove(&self, ids: Vec<String>, delete_local_data: bool);
    /// Get session statistics.
    async fn stats(&self) -> SessionStats;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SessionStats {
    pub active_torrent_count: i32,
    pub cumulative_stats: StatsDetails,
    pub current_stats: StatsDetails,
    pub download_speed: i32,
    pub paused_torrent_count: i32,
    pub torrent_count: i32,
    pub upload_speed: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StatsDetails {
    pub downloaded_bytes: i64,
    pub files_added: i64,
    pub seconds_active: i64,
    pub session_count: i64,
    pub uploaded_bytes: i64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Torrent {
    pub id: i32,
    pub activity_date: i32,
    pub added_date: i32,
    pub bandwidth_priority: i32,
    pub comment: String,
    pub corrupt_ever: i64,
    pub creator: String,
    pub date_created: i32,
    pub desired_available: i64,
    pub done_date: i32,
    pub download_dir: String,
    pub download_limit: i32,
    pub download_limited: bool,
    pub downloaded_ever: i64,
    pub edit_date: i32,
    pub error: i32,
    pub error_string: String,
    pub eta: i64,
    pub eta_idle: i64,
    pub hash_string: String,
    pub have_unchecked: i64,
    pub have_valid: i64,
    pub honors_session_limits: bool,
    pub is_finished: bool,
    pub is_private: bool,
    pub is_stalled: bool,
    pub left_until_done: i64,
    pub magnet_link: String,
    pub manual_announce_time: i32,
    pub metadata_percent_complete: f32,
    pub name: String,
    pub percent_done: f32,
    pub piece_count: i64,
    pub piece_size: i64,
    pub pieces: String,
    pub primary_mime_type: String,
    pub queue_position: i32,
    pub rate_download: i32,
    pub rate_upload: i32,
    pub recheck_progress: f32,
    pub seconds_downloading: i32,
    pub seconds_seeding: i32,
    pub seed_idle_limit: i32,
    pub seed_idle_mode: i32,
    pub seed_ratio_limit: f32,
    pub seed_ratio_mode: i32,
    pub size_when_done: i64,
    pub start_date: i32,
    pub status: i32,
    pub torrent_file: String,
    pub total_size: i64,
    pub upload_limit: i32,
    pub upload_limited: bool,
    pub upload_ratio: f32,
    pub uploaded_ever: i64,
}
