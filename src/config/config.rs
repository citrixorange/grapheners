use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::iter::FromIterator;

lazy_static! {
    pub static ref CONFIG: Config = {
        // Open the file
        let file_path = "../../config.json";
        let mut file = File::open(file_path).expect("Unable to open config file");

        // Read the file contents into a string
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Unable to read config file");

        // Deserialize the JSON data into your config struct
        serde_json::from_str(&file_contents).expect("Unable to parse config JSON")
    };
}

#[derive(Debug, Deserialize)]
pub struct Config {
    custom_apis: Vec<CustomApi>
}

impl Config {
    pub fn get_custom_api_name(&self, id:u8) -> String {
        return self.custom_apis
            .iter()
            .find(|api| api.id == id)
            .map(|api| api.name.clone())
            .unwrap_or(String::from(""))
    }
}

#[derive(Clone, Debug, Deserialize)]
struct CustomApi {
    pub id: u8,
    pub name: String
}

impl<'a> FromIterator<&'a CustomApi> for Vec<CustomApi> {
    fn from_iter<I: IntoIterator<Item = &'a CustomApi>>(iter: I) -> Self {
        iter.into_iter().cloned().collect()
    }
}