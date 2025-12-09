use transmission_test::{torrent::BitTorrent, *};

#[tokio::main]
async fn main() {
    let client = transmission::TransmissionClient::new(None);
    let torrent = client
        .add(
            "/home/jos/Downloads/test_folder-d984f67af9917b214cd8b6048ab5624c7df6a07a.torrent",
            "/home/jos/tmp/torrents/",
        )
        .await;
    println!("Added torrent: {:?}", torrent);
    let torrents = client.list().await;
    println!("Torrents: {:?}", torrents);
}
