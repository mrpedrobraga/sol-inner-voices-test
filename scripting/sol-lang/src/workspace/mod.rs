pub mod build;
pub mod init;

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
enum CliError {
    IO(#[from] std::io::Error),
}
