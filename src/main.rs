mod torrent;
mod tracker;
use tracker::Tracker;
use torrent::TorrentFile;

fn main() {
    let _t = TorrentFile::new(String::from("test2.torrent"));
    todo!("finish");
}
