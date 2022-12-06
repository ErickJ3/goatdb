use serde::Serialize;
use std::{
    collections::HashMap,
    fs::{self},
    io::{self, ErrorKind},
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

        let exist = Path::new(&db_path_buf).exists();

        if exist == false {
            fs::File::create(&db_path_buf).unwrap();
        }

        let mut map_database: HashMap<String, Vec<u8>> = HashMap::new();

        let load_data = match self::GoatDb::load_data(db_path_buf.clone()) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };

        for (key, value) in load_data.iter() {
            map_database.insert(key.to_string(), value.to_vec());
        }

        let map_db = match load_data.is_empty() {
            true => HashMap::new(),
            false => load_data,
        };

        GoatDb {
            map: map_db,
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

    pub fn get(&self, key: &str) -> Option<String> {
        let value: Option<Vec<u8>> = match self.map.get(key) {
            Some(data) => Some(data.to_vec()),
            _ => None,
        };

        if let None = value {
            Some(String::from("key not exist"))
        } else {
            let value_vec = value.unwrap();

            let value_to_utf8 = std::str::from_utf8(&value_vec).unwrap();

            let deserialize = value_to_utf8.to_string();

            Some(deserialize)
        }
    }

    fn load_data(db_file_path: PathBuf) -> Result<HashMap<String, Vec<u8>>, io::Error> {
        let content = fs::read(db_file_path).unwrap();

        if content.is_empty() {
            return Ok(HashMap::new());
        }

        match serde_json::from_str::<HashMap<String, String>>(
            std::str::from_utf8(&content).unwrap(),
        ) {
            Ok(json_map) => {
                let mut byte_map = HashMap::new();

                for (key, value) in json_map.iter() {
                    byte_map.insert(key.to_string(), value.as_bytes().to_vec());
                }

                Ok(byte_map)
            }

            Err(_) => Err(io::Error::new(
                ErrorKind::Interrupted,
                "error in load database!",
            )),
        }
    }
}
