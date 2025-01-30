use homedir::my_home;
use std::{collections::HashSet, fs, fs::File, io::Write};

static DIR_NAME: &str = "Documents";
static DATA_FILE_NAME: &str = "input.txt";

pub const MIN_WORD_SIZE: usize = 10;

pub fn build_data_file_path() -> Result<String, &'static str> {
    let mut result = Err("Unable to retrieve the data file path");

    if let Ok(Some(mut path)) = my_home() {
        path.push(DIR_NAME);
        path.push(DATA_FILE_NAME);

        if let Some(path) = path.to_str() {
            result = Ok(path.to_string());
        }
    }

    result
}

pub struct IOManager {
    data_file: String,
}

impl IOManager {
    pub fn create(file_path: &str) -> IOManager {
        IOManager {
            data_file: file_path.to_string(),
        }
    }

    pub fn load(&self) -> Result<Vec<String>, &'static str> {
        let mut result = Err("File could not be opened");

        if let Ok(data_str) = fs::read_to_string(&self.data_file) {
            // use HashSet to remove duplicate values, the collect the (valid) data into a vector for enabling randomization
            let data: HashSet<String> = data_str
                .split('\n')
                .filter(|word| {
                    // although len() returns number of bytes (not chars) if we constrain the chars to be ASCII then it should represent the number of chars (otherwise if not ASCII the filter will fail anyway)
                    word.chars().all(|ch| ch.is_ascii_alphabetic()) && word.len() >= MIN_WORD_SIZE
                })
                .map(|val| {
                    val.chars()
                        .map(|ch| ch.to_ascii_lowercase())
                        .collect::<String>()
                })
                .collect();

            result = if !data.is_empty() {
                Ok(data.iter().map(|val| val.to_string()).collect())
            } else {
                Err("File is empty or contains no valid data")
            };
        }

        result
    }

    pub fn save(&self, data: Vec<String>) -> Result<usize, &'static str> {
        let mut result = Err("Empty content, nothing to save");

        if !data.is_empty() {
            let file = File::create(&self.data_file);

            match file {
                Ok(mut file_handle) => {
                    let words_to_save_count = data.len();
                    let mut saved_words_count = 0;

                    for word in data.into_iter() {
                        if let Ok(_res) = file_handle.write_all((word + "\n").as_bytes()) {
                            saved_words_count += 1;
                            continue;
                        }

                        break;
                    }

                    result = if saved_words_count == words_to_save_count {
                        Ok(saved_words_count)
                    } else {
                        Err("An error occurred while saving the words")
                    };
                }
                Err(_err) => {
                    result = Err("Unable to open the file for saving");
                }
            }
        }

        result
    }

    pub fn get_data_file(&self) -> &str {
        &self.data_file
    }
}
