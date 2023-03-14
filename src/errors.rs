use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("{0} could not be parsed")]
    FileParseFailed(String),

    #[error(".trashinfo file at {0} is missing its corresponding trashed file")]
    TrashInfoMissing(String),

    #[error(".trashinfo at {0} is invalid, could not parse")]
    TrashInfoParseFailure(String),

    #[error("non-unicode path could not be used as a key for a .trashinfo file")]
    PathInvalidUnicode,

    #[error("could not determine filesystem that contains $XDG_DATA_HOME/Trash, found {0} results")]
    HomeTrashNotDetermined(usize),
}
