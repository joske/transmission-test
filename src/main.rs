mod torrent;
mod transmission;

use torrent::BitTorrent;

#[tokio::main]
async fn main() {
    let client = transmission::TransmissionClient::new(None);
    let stats = client.stats().await;
    println!("{:#?}", stats);
}
