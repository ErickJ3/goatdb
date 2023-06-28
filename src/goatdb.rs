use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, ErrorKind, Read},
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

        let exist = db_path_buf.exists();

        if !exist {
            File::create(&db_path_buf).expect("Failed to create database file");
        }

        let load_data = match Self::load_data(db_path_buf.clone()) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        let map_database = load_data
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_vec()))
            .collect();

        GoatDb {
            map: map_database,
            db_file_path: db_path_buf,
        }
    }

    pub fn set<V>(&mut self, key: &str, value: &V)
    where
        V: Serialize,
    {
        self.map
            .insert(key.to_string(), serde_json::to_vec(value).unwrap());

        let json_map: HashMap<&str, &str> = self
            .map
            .iter()
            .map(|(key, value)| (key.as_str(), std::str::from_utf8(value).unwrap()))
            .collect();

        let serde_json_data = serde_json::to_string(&json_map).unwrap();

        if let Err(err) = fs::write(&self.db_file_path, serde_json_data) {
            println!("error: {}", err);
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let value = self.map.get(key).map(|data| data.to_vec());

        value
            .and_then(|value_vec| {
                std::str::from_utf8(&value_vec)
                    .ok()
                    .map(|value_utf8| value_utf8.to_string())
            })
            .or_else(|| Some(String::from("key not exist")))
    }

    fn load_data(db_file_path: PathBuf) -> Result<HashMap<String, Vec<u8>>, io::Error> {
        let mut content = Vec::new();
        File::open(db_file_path)?.read_to_end(&mut content)?;

        if content.is_empty() {
            return Ok(HashMap::new());
        }

        match serde_json::from_slice::<HashMap<String, String>>(&content) {
            Ok(json_map) => {
                let byte_map = json_map
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.as_bytes().to_vec()))
                    .collect();

                Ok(byte_map)
            }

            Err(_) => Err(io::Error::new(
                ErrorKind::Interrupted,
                "error in load database!",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let db_path = "test_db.json";
        let mut goat_db = GoatDb::new(db_path);

        goat_db.set("key1", &42);
        goat_db.set("key2", &"value");

        assert_eq!(goat_db.get("key1"), Some("42".to_string()));
        assert_eq!(goat_db.get("key2"), Some("\"value\"".to_string()));
        assert_eq!(goat_db.get("key3"), Some("key not exist".to_string()));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let db_path = "test_db.json";
        let goat_db = GoatDb::new(db_path);

        assert_eq!(
            goat_db.get("nonexistent_key"),
            Some("key not exist".to_string())
        );
    }

    #[test]
    fn test_load_empty_db() {
        let db_path = "empty_db.json";
        let goat_db = GoatDb::new(db_path);

        assert_eq!(goat_db.get("any_key"), Some("key not exist".to_string()));
    }
}
