use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
};

pub struct GoatDb {
    map: HashMap<String, Vec<u8>>,
    db_file_path: PathBuf,
}

impl GoatDb {
    pub fn new<P: AsRef<Path>>(db_path: P) -> GoatDb {
        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        GoatDb {
            map: HashMap::new(),
            db_file_path: db_path_buf,
        }
    }

    pub fn set<V>(&mut self, key: &str, value: &V)
    where
        V: Serialize,
    {
        let binding = serde_json::to_string(value).unwrap();
        let data = binding.as_bytes();

        self.map.insert(String::from(key), data.to_vec());

        let mut json_map: HashMap<&str, &str> = HashMap::new();

        for (key, value) in self.map.iter() {
            json_map.insert(key, std::str::from_utf8(value).unwrap());
        }

        let serde_json_data = serde_json::to_string(&(json_map)).unwrap();

        match fs::write(&self.db_file_path, serde_json_data) {
            Ok(_) => (),
            Err(err) => println!("error: {}", err),
        }
    }

    pub fn get(&self, key: &str) -> String {
        let value = self.map.get(key).unwrap();

        let value_to_utf8 = std::str::from_utf8(value).unwrap();

        let deserialize = serde_json::from_str(value_to_utf8).unwrap();

        deserialize
    }
}
