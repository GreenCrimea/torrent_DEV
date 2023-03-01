use std::{fmt::Error};
use serde_json::{Result, Value};
use bencode_encoder::Decoder;

fn main() {
    todo!("finish");
}



struct TorrentFile {
    torrent_filepath: String,
    decoded_torrent: Value,
    total_length: i32,
    piece_length: i64,
    pieces: i64,
    info_hash: String,
    peer_id: String,
    announce_list: Vec<String>,
    file_names: Vec<String>,
    number_of_pieces: i32,
}

impl TorrentFile {
    fn new(file_path: String) -> Result<TorrentFile> {
        let decoded_torrent = Self::load_from_path(&file_path);
        let torrent_filepath = file_path;
        let piece_length = decoded_torrent["info"]["piece length"].as_i64().unwrap();
        let pieces = decoded_torrent["info"]["pieces"].as_i64().unwrap();
        

        Ok(TorrentFile {torrent_filepath, 
                        decoded_torrent, 
                        total_length: (), 
                        piece_length, 
                        pieces, 
                        info_hash: (), 
                        peer_id: (), 
                        announce_list: (), 
                        file_names: (), 
                        number_of_pieces: () })
    }

    fn load_from_path(file_path: &String) -> Value {
        let decoded = Decoder::decode_from(file_path).unwrap();
        let string_json = decoded.to_json().unwrap();
        let json = serde_json::from_str(&string_json).unwrap();
        json
    }

    
}


