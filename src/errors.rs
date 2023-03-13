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
}
