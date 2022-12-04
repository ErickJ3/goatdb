use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self},
    path::{Path, PathBuf},
};

pub struct GoatDb {
    map: HashMap<String, String>,
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
        let data = serde_json::to_string(value).unwrap();

        self.map.insert(String::from(key), data);

        let mut key_and_values = Vec::new();

        for (k, v) in self.map.iter() {
            key_and_values.push(format!("{}:{}", k, v));
        }

        let input = serde_json::to_string(&key_and_values).unwrap();

        match fs::write(&self.db_file_path, input) {
            Ok(_) => (),
            Err(err) => println!("error: {}", err),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.map.get(key) {
            Some(val) => Some(val.to_string()),
            None => None,
        }
    }
}
