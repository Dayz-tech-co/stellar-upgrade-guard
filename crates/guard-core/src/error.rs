use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum GuardError {
    #[error("failed to read WASM file {path}: {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid WASM: {0}")]
    InvalidWasm(String),

    #[error("missing contractspecv0 custom section")]
    MissingContractSpec,

    #[error("invalid contractspecv0 XDR stream: {0}")]
    InvalidContractSpec(String),
}
