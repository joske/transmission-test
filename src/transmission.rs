use transmission_client::{
    Client, SessionMutator, SessionStats as TransmissionSessionStats,
    StatsDetails as TransmissionStatsDetails, Torrent as TransmissionTorrent,
};
use url::Url;

use crate::torrent::{BitTorrent, SessionStats, StatsDetails, Torrent};

pub struct TransmissionClient {
    client: Client,
}

impl TransmissionClient {
    /// Create a new TransmissionClient.
    /// If no RPC URL is provided, it defaults to "http://localhost:9091/transmission/rpc".
    /// This method is async as the session settings are applied on creation.
    /// An incomplete directory can also be specified. If None is provided, it defaults to "/tmp/transmission/incomplete".
    pub async fn new(
        rpc_url: Option<&str>,
        incomplete_dir: Option<&str>,
        max_downloads: u32,
    ) -> Self {
        let url = Url::parse(rpc_url.unwrap_or("http://localhost:9091/transmission/rpc")).unwrap();
        let client = Client::new(url);
        let incomplete_dir = incomplete_dir.unwrap_or("/tmp/transmission/incomplete");
        let session_mutator = SessionMutator {
            incomplete_dir_enabled: Some(true),
            incomplete_dir: Some(incomplete_dir.into()),
            download_queue_enabled: Some(true),
            download_queue_size: Some(max_downloads as i32),
            ..Default::default()
        };
        client.session_set(session_mutator).await.unwrap();
        Self { client }
    }
}

impl BitTorrent for TransmissionClient {
    async fn add(&self, torrent_file: &str, download_dir: &str) -> Torrent {
        self.client
            .torrent_add_filename_download_dir(torrent_file, download_dir)
            .await
            .unwrap()
            .unwrap()
            .into()
    }

    async fn stop(&self, ids: Vec<String>) {
        self.client.torrent_stop(Some(ids)).await.unwrap();
    }

    async fn list(&self) -> Vec<Torrent> {
        self.client
            .torrents(None)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect()
    }

    async fn remove(&self, ids: Vec<String>, delete_local_data: bool) {
        self.client
            .torrent_remove(Some(ids), delete_local_data)
            .await
            .unwrap();
    }

    async fn stats(&self) -> SessionStats {
        self.client.session_stats().await.unwrap().into()
    }
}

// conversions from transmission_client types to our defined types

impl From<TransmissionSessionStats> for SessionStats {
    fn from(value: TransmissionSessionStats) -> Self {
        Self {
            active_torrent_count: value.active_torrent_count,
            cumulative_stats: value.cumulative_stats.into(),
            current_stats: value.current_stats.into(),
            download_speed: value.download_speed,
            paused_torrent_count: value.paused_torrent_count,
            torrent_count: value.torrent_count,
            upload_speed: value.upload_speed,
        }
    }
}

impl From<TransmissionStatsDetails> for StatsDetails {
    fn from(value: TransmissionStatsDetails) -> Self {
        Self {
            downloaded_bytes: value.downloaded_bytes,
            files_added: value.files_added,
            seconds_active: value.seconds_active,
            session_count: value.session_count,
            uploaded_bytes: value.uploaded_bytes,
        }
    }
}

impl From<TransmissionTorrent> for Torrent {
    fn from(value: TransmissionTorrent) -> Self {
        Self {
            id: value.id,
            activity_date: value.activity_date,
            added_date: value.added_date,
            bandwidth_priority: value.bandwidth_priority,
            comment: value.comment,
            corrupt_ever: value.corrupt_ever,
            creator: value.creator,
            date_created: value.date_created,
            desired_available: value.desired_available,
            done_date: value.done_date,
            download_dir: value.download_dir,
            download_limit: value.download_limit,
            download_limited: value.download_limited,
            downloaded_ever: value.downloaded_ever,
            edit_date: value.edit_date,
            error: value.error,
            error_string: value.error_string,
            eta: value.eta,
            eta_idle: value.eta_idle,
            hash_string: value.hash_string,
            have_unchecked: value.have_unchecked,
            have_valid: value.have_valid,
            honors_session_limits: value.honors_session_limits,
            is_finished: value.is_finished,
            is_private: value.is_private,
            is_stalled: value.is_stalled,
            left_until_done: value.left_until_done,
            magnet_link: value.magnet_link,
            manual_announce_time: value.manual_announce_time,
            metadata_percent_complete: value.metadata_percent_complete,
            name: value.name,
            percent_done: value.percent_done,
            piece_count: value.piece_count,
            piece_size: value.piece_size,
            pieces: value.pieces,
            primary_mime_type: value.primary_mime_type,
            queue_position: value.queue_position,
            rate_download: value.rate_download,
            rate_upload: value.rate_upload,
            recheck_progress: value.recheck_progress,
            seconds_downloading: value.seconds_downloading,
            seconds_seeding: value.seconds_seeding,
            seed_idle_limit: value.seed_idle_limit,
            seed_idle_mode: value.seed_idle_mode,
            seed_ratio_limit: value.seed_ratio_limit,
            seed_ratio_mode: value.seed_ratio_mode,
            size_when_done: value.size_when_done,
            start_date: value.start_date,
            status: value.status,
            torrent_file: value.torrent_file,
            total_size: value.total_size,
            upload_limit: value.upload_limit,
            upload_limited: value.upload_limited,
            upload_ratio: value.upload_ratio,
            uploaded_ever: value.uploaded_ever,
        }
    }
}
