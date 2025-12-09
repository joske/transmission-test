use transmission_test::{torrent::BitTorrent, *};

#[tokio::main]
async fn main() {
    let client = transmission::TransmissionClient::new(None, None).await;
    let torrent = client
        .add(
            "/home/jos/Downloads/GhostBSD-25.02-R14.3p2-GERSHWIN.iso.torrent",
            "/home/jos/tmp/torrents/",
        )
        .await;
    println!("Added torrent: {:?}", torrent);
    let torrents = client.list().await;
    println!("Torrents: {:?}", torrents);
}
