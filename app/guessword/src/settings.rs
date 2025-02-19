use crate::errors::IOError;

static DIR_NAME: &str = "Documents";
static DATA_FILE_NAME: &str = "input.txt";

pub const MIN_WORD_SIZE: usize = 10;

pub fn build_data_file_path() -> Result<String, IOError> {
    let mut result = Err(IOError::CannotRetrieveFilePath);

    if let Ok(Some(mut path)) = homedir::my_home() {
        path.push(DIR_NAME);
        path.push(DATA_FILE_NAME);

        if let Some(path) = path.to_str() {
            result = Ok(path.to_string());
        }
    }

    result
}
