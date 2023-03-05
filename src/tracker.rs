use std::collections::HashMap;

use crate::torrent::TorrentFile;



pub struct Tracker {
    torrent: TorrentFile,
    threads_list: Vec<String>,
    connected_peers: HashMap<String, Vec<u8>>,
    dict_sock_addr: HashMap<String, Vec<u8>>,
}

impl Tracker {
    pub fn new(torrent: TorrentFile) -> Tracker {
               
        Tracker { torrent,
                  threads_list: Vec::new(), 
                  connected_peers: HashMap::new(), 
                  dict_sock_addr: HashMap::new() 
        }
    }

    pub fn get_peers_from_trackers() -> () {
        todo!("finish")
    }

    pub fn try_peer_connect() -> () {
        todo!("finish")
    }

    pub fn http_scraper() -> () {
        todo!("finish")
    }

    pub fn udp_scraper() -> () {
        todo!("finish")
    }

    pub fn send_message() -> () {
        todo!("finish")
    }
}