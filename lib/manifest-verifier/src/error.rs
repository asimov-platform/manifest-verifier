use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Path is not relative")]
    PathNotRelative,

    #[error("Failed to read file")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse YAML")]
    ParseYAML(#[from] marked_yaml::FromYamlError),
}
