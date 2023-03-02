use std::{fmt::Error, collections::HashMap};
use serde_json::{Result, Value};
use bencode_encoder::Decoder;

fn main() {
    todo!("finish");
}



struct TorrentFile {
    torrent_filepath: String,
    decoded_torrent: Value,
    total_length: i64,
    piece_length: i64,
    pieces: i64,
    info_hash: String,
    peer_id: String,
    announce_list: Vec<String>,
    file_names: HashMap<String, String>,
    number_of_pieces: i32,
}

impl TorrentFile {
    fn new(file_path: String) -> Result<TorrentFile> {
        let decoded_torrent = Self::load_from_path(&file_path);
        let torrent_filepath = file_path;
        let piece_length = decoded_torrent["info"]["piece length"].as_i64().unwrap();
        let pieces = decoded_torrent["info"]["pieces"].as_i64().unwrap();
        let (file_names, total_length) = Self::init_files(&decoded_torrent);

        Ok(TorrentFile {torrent_filepath, 
                        decoded_torrent, 
                        total_length, 
                        piece_length, 
                        pieces, 
                        info_hash: (), 
                        peer_id: (), 
                        announce_list: (), 
                        file_names, 
                        number_of_pieces: () })
    }

    fn load_from_path(file_path: &String) -> Value {
        let decoded = Decoder::decode_from(file_path).unwrap();
        let string_json = decoded.to_json().unwrap();
        let json = serde_json::from_str(&string_json).unwrap();
        json
    }

    fn init_files(decoded_torrent: &Value) -> (HashMap<String, String>, i64) {

        let root = decoded_torrent["info"]["name"].to_string();
        let mut file_names: HashMap<String, String> = HashMap::new();

        if decoded_torrent["info"]["name"].is_object() {

            let files = decoded_torrent["info"]["name"];
            todo!("finish");

        } else {
            file_names.insert("path".to_owned(), root.to_owned());
            file_names.insert("length".to_owned(), decoded_torrent["info"]["length"].to_string().to_owned());
            let total_length = decoded_torrent["info"]["Length"].as_i64().unwrap();
            (file_names, total_length)
        }
    }

    
}


