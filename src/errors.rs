use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("{0} could not be parsed")]
    FileParseFailed(String),

    #[error(".trashinfo at {0} is invalid, could not parse")]
    TrashInfoParseFailure(String),

    #[error("could not determine filesystem that contains $XDG_DATA_HOME/Trash, found {0} results")]
    HomeTrashNotDetermined(usize),

    #[error("{0} could not be trashed because the file does not exist")]
    BadQueryFileNotFound(String),

    #[error("could not determine trash directory for {0}")]
    TrashDirNotFound(String),

    #[error("trashing of {0} could not be completed, because creation of directory {0} failed")]
    CreateDirectoryFailed(String, String),

    #[error("unable to find suitable name for {0}")]
    UnableToFindSuitableName(String),

    #[error("could not get file name from path: {0}")]
    CouldNotRetrieveFileName(String),

    #[error("the file to trash, {0}, is not relative to the parent of trash directory at {1}")]
    FileNotRelative(String, String),
}
