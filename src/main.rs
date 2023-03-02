use bencode_encoder::Decoder;
use serde_json::{Result, Value};
use std::{
    collections::HashMap,
    fs::create_dir,
    path::{Path, PathBuf},
};

fn main() {
    let _t = TorrentFile::new(String::from("test.torrent"));
    todo!("finish");
}

struct TorrentFile {
    torrent_filepath: String,
    decoded_torrent: Value,
    total_length: i64,
    piece_length: i64,
    pieces: String,
    //info_hash: String,
    //peer_id: String,
    announce_list: Vec<String>,
    file_names: Vec<HashMap<String, String>>,
    //number_of_pieces: i32,
}

impl TorrentFile {
    fn new(file_path: String) -> Result<TorrentFile> {
        let decoded_torrent = Self::load_from_path(&file_path);
        let torrent_filepath = file_path;
        let piece_length = decoded_torrent["info"]["piece length"]
            .as_i64()
            .expect("failed to get piece length from decoded torrent");
        let pieces = decoded_torrent["info"]["pieces"].to_string();
        let (file_names, total_length) = Self::init_files(&decoded_torrent);
        let announce_list: Vec<String> = Self::get_trackers(&decoded_torrent);

        Ok(TorrentFile {
            torrent_filepath,
            decoded_torrent,
            total_length,
            piece_length,
            pieces,
            //info_hash,
            //peer_id,
            announce_list,
            file_names,
            //number_of_pieces
        })
    }

    fn load_from_path(file_path: &String) -> Value {
        serde_json::from_str(&Decoder::decode_from(file_path).unwrap().to_json().unwrap()).unwrap()
    }

    fn init_files(decoded_torrent: &Value) -> (Vec<HashMap<String, String>>, i64) {
        let root = decoded_torrent["info"]["name"].to_string();
        let mut file_names: Vec<HashMap<String, String>> = Vec::new();
        let mut file_l: HashMap<String, String> = HashMap::new();

        if decoded_torrent["info"]["files"].is_array() {
            let files = decoded_torrent["info"]["files"].as_array().unwrap();

            if !Path::new(&root).is_dir() {
                create_dir(&root).unwrap();
            }
            let mut total_length: i64 = 0;

            for file in files {
                let path_list = file.as_object().unwrap()["path"].as_array().unwrap();
                let mut path_buf = PathBuf::new();
                let mut file_list = file_l.clone();

                if path_list.len() == 1 {
                    file_list.insert(
                        "path".to_owned(),
                        path_list.first().unwrap().as_str().unwrap().to_string(),
                    );
                    file_list.insert("length".to_owned(), file["length"].to_string().to_owned());

                } else {
                    let i = path_list.len();

                    for j in 0..i - 1 {
                        path_buf.push(path_list.get(j).unwrap().as_str().unwrap());
                    }
                    let mut path_complete = path_buf.clone();
                    path_complete.push(path_list.get(i - 1).unwrap().as_str().unwrap());
                    
                    let path_c = path_complete.to_str().unwrap();
                    
                    file_list.insert("path".to_owned(), path_c.to_string());
                    file_list.insert("length".to_owned(), file["length"].to_string().to_owned());
                }
                if !Path::new(&root).join(&path_buf).is_dir() {
                    create_dir(Path::new(&root).join(&path_buf)).unwrap();
                }
                file_names.push(file_list);
                total_length += file["length"].as_i64().unwrap();
            }
            (file_names, total_length)

        } else {
            file_l.insert("path".to_owned(), root.to_owned());
            file_l.insert(
                "length".to_owned(),
                decoded_torrent["info"]["length"].to_string().to_owned(),
            );
            file_names.push(file_l);
            let total_length = decoded_torrent["info"]["Length"].as_i64().unwrap();

            (file_names, total_length)
        }
    }

    fn get_trackers(decoded_torrent: &Value) -> Vec<String> {
        
        todo!("finish")
    }

}

