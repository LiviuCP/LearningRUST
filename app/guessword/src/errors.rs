#[derive(PartialEq, Debug, Clone)]
pub enum IOError {
    CannotLoadFile,
    CannotSaveFile,
    CannotRetrieveFilePath,
    OtherSavingError,
    UserInputError,
}
