use std::path::Path;
use toml_edit::{Document, Item, Key, Table, value};

fn add_uuid<P: AsRef<Path>>(toml_path: P) {
    let mut doc = std::fs::read_to_string(toml_path.as_ref())
        .expect("Failed to read toml to string")
        .parse::<Document>()
        .expect("Invalid toml document");
    if !doc["album"].as_table().unwrap().contains_key("album_id") {
        let mut album = Table::new();
        album.set_position(0);
        album["album_id"] = value(uuid::Uuid::new_v4().to_string());
        for (k, v) in doc["album"].as_table().unwrap().clone().into_iter() {
            album.insert_formatted(&Key::new(k), v);
        }
        doc["album"] = Item::Table(album);
        // remove prefix \n, append \n
        let result = format!("{}\n", doc.to_string().trim());
        std::fs::write(toml_path.as_ref(), result).expect("Failed to write toml");
    }
}

fn main() {
    let dirs = std::fs::read_dir("/home/yesterday17/Code/Music/repo/album").unwrap();
    for dir in dirs {
        let dir = dir.unwrap();
        add_uuid(dir.path());
    }
}
